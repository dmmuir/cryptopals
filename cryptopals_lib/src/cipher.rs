extern crate openssl;

use super::blocks::Blocks;
use super::heuristics::contain_duplicates;
use super::xor;

#[derive(Debug, PartialEq)]
pub enum EncryptionMode {
    ECB,
    CBC,
}

pub fn ecb_mode_encrypt(data: &[u8], key: &[u8]) -> Vec<u8> {
    use openssl::symm::{encrypt, Cipher};
    let cipher = Cipher::aes_128_ecb();

    encrypt(cipher, key, None, data)
        .expect("Failed to encrypt ecb mode encryption with specified key")
}

pub fn ecb_mode_decrypt(data: &[u8], key: &[u8]) -> Vec<u8> {
    use openssl::symm::{decrypt, Cipher};
    let cipher = Cipher::aes_128_ecb();

    decrypt(cipher, key, None, data)
        .expect("Failed to decrypt ecb mode decryption with specified key")
}

pub fn detect_encryption_mode(data: &[u8]) -> EncryptionMode {
    let blocks = Blocks::from(16, data).chunk_slice();

    if contain_duplicates(&blocks) {
        EncryptionMode::ECB
    } else {
        EncryptionMode::CBC
    }
}

pub fn detect_ecb_mode_encryption(data: &[Vec<u8>]) -> Vec<(usize, Vec<u8>)> {
    data.iter()
        .enumerate()
        .filter(|(_index, line)| {
            let blocks = Blocks::from(16, line);
            let blocks = blocks.chunk_slice();
            contain_duplicates(&blocks)
        })
        .map(|(index, line)| (index, line.to_owned()))
        .collect()
}

pub fn cbc_mode_encrypt(data: &[u8], key: &[u8], iv: &[u8]) -> Vec<u8> {
    let padded_data = Blocks::with_padding_from(16, data);
    let mut prev_block = iv.to_vec();

    padded_data
        .into_iter()
        .map(|block| {
            let xord_plaintext = xor::fixed_xor(&block, &prev_block);
            let encrypt_message = ecb_mode_encrypt(&xord_plaintext, key)[..16].to_vec();
            prev_block = encrypt_message.clone();

            encrypt_message
        })
        .flatten()
        .collect()
}

pub fn cbc_mode_decrypt(data: &[u8], key: &[u8], iv: &[u8]) -> Vec<u8> {
    let padded_data = Blocks::with_padding_from(16, data);
    let mut prev_block = iv.to_vec();
    let ecb_mode_salt: Vec<u8> = vec![
        96, 250, 54, 112, 126, 69, 244, 153, 219, 160, 242, 91, 146, 35, 1, 165,
    ];

    padded_data
        .into_iter()
        .map(|block| {
            let salty_block = [block.clone(), ecb_mode_salt.clone()].concat();
            let decrypt_message = ecb_mode_decrypt(&salty_block, key);
            let plaintext = xor::fixed_xor(&decrypt_message, &prev_block);
            prev_block = block;

            plaintext
        })
        .flatten()
        .collect()
}


#[cfg(test)]
mod test {
    use super::*;

    mod ecb {
        use super::*;

        #[test]
        fn encrypt_decrypt() {
            let data = b"Figuring to decrypt ecb mode encryption with key and back again!"; // 64 bytes long -> 4 blocks of 16 bytes
            let key = b"YELLOW SUBMARINE";

            let secret = ecb_mode_encrypt(data, key);
            let message = ecb_mode_decrypt(&secret, key);
            assert_eq!(data.to_vec(), message)
        }
    }

    mod cbc {
        use super::*;

        #[test]
        fn encrypt_decrypt() {
            let data = b"Figuring to decrypt ecb mode encryption with key and back again!"; // 64 bytes long -> 4 blocks of 16 bytes
            let key = b"YELLOW SUBMARINE";
            let iv = vec![b'0'; 16];

            let secret = cbc_mode_encrypt(data, key, &iv);
            let message = cbc_mode_decrypt(&secret, key, &iv);
            assert_eq!(data.to_vec(), message)
        }
    }

    mod detection {
        use super::*;

        #[test]
        fn ecb() {
            let data = b"Figuring to decrypt ecb mode encryption with key and back again!Figuring to decrypt ecb mode encryption with key and back again!"; // 64 bytes long -> 4 blocks of 16 bytes
            let key = b"YELLOW SUBMARINE";

            let secret = ecb_mode_encrypt(data, key);
            assert_eq!(EncryptionMode::ECB, detect_encryption_mode(&secret))
        }

        #[test]
        fn cbc() {
            let data = b"Figuring to decrypt ecb mode encryption with key and back again!Figuring to decrypt ecb mode encryption with key and back again!"; // 64 bytes long -> 4 blocks of 16 bytes
            let key = b"YELLOW SUBMARINE";
            let iv = vec![b'0'; 16];

            let secret = cbc_mode_encrypt(data, key, &iv);
            assert_eq!(EncryptionMode::CBC, detect_encryption_mode(&secret))
        }
    }
}
