extern crate openssl;

use super::blocks::Blocks;
use super::heuristics::contain_duplicates;

pub fn ecb_mode_decrypt(data: &[u8], key: &[u8]) -> Vec<u8> {
    use openssl::symm::{decrypt, Cipher};
    let cipher = Cipher::aes_128_ecb();

    decrypt(cipher, key, None, &data)
        .expect("Failed to decrypt ecb mode encryption with specified key")
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
