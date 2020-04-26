pub mod base64;
pub mod blocks;
pub mod cipher;
pub mod heuristics; // TODO: Module needs a better name
pub mod hex;
pub mod witchcraft;
pub mod xor;

#[cfg(test)]
mod set1 {
    use super::*;

    mod challenge1 {
        use super::*;

        #[test]
        fn convert_hex_to_base64() {
            let from_hex_bytes = hex::decode(
                b"49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d"
            );

            let base64_str =
                b"SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t".to_vec();

            assert_eq!(base64_str, base64::encode(&from_hex_bytes))
        }
    }

    mod challenge2 {
        use super::*;

        #[test]
        fn fixed_xor() {
            let a = hex::decode(b"1c0111001f010100061a024b53535009181c");
            let b = hex::decode(b"686974207468652062756c6c277320657965");
            let expected = b"746865206b696420646f6e277420706c6179".to_vec();

            let actual = xor::fixed_xor(&a, &b);
            assert_eq!(expected, hex::encode(&actual))
        }
    }

    mod challenge3 {
        use super::*;

        #[test]
        fn single_byte_xor() {
            let slice = hex::decode(
                b"1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736",
            );
            let actual = xor::decrypt_single_byte_xor(&slice);

            assert_eq!(
                String::from("Cooking MC's like a pound of bacon"),
                String::from_utf8(actual).unwrap()
            )
        }
    }

    mod challenge4 {
        use super::*;

        #[test]
        fn detect_single_character_xor() {
            let slice = hex::decode(b"7b5a4215415d544115415d5015455447414c155c46155f4058455c5b52");
            let (_index, key) = &xor::detect_single_byte_xor_line(&[slice.clone()]);
            let actual = xor::single_byte_xor(*key)(&slice);

            assert_eq!(
                String::from("Now that the party is jumping"),
                String::from_utf8(actual.to_owned()).unwrap()
            )
        }
    }

    mod challenge5 {
        use super::*;

        #[test]
        fn repeating_key_xor() {
            let message =
                "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal";
            let key = b"ICE";
            let expected = [
                b"0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272"
                    .to_vec(),
                b"a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f"
                    .to_vec(),
            ]
            .concat();

            let encrypt = xor::repeating_key_xor(key);
            let secret = encrypt(message.as_bytes());

            assert_eq!(expected, hex::encode(&secret))
        }
    }

    mod challenge6 {
        use super::*;

        #[test]
        fn _find_key() {
            let message = base64::decode(&_file_reader("../challenge-data/6.txt"));

            let expected_key = b"Terminator X: Bring the noise".to_vec();

            let key = xor::find_key(&message);
            assert_eq!(expected_key, key);
        }

        #[test]
        fn _decrypt_repeating_key_xor() {
            let message = base64::decode(&_file_reader("../challenge-data/6.txt"));

            assert_eq!(
                std::fs::read_to_string("../test-data/6-test.txt")
                    .unwrap()
                    .as_bytes()
                    .to_vec(),
                xor::decrypt_repeating_key_xor(&message),
            );
        }
    }
}

#[cfg(test)]
mod set2 {
    use super::*;
    
    mod challenge9 {
        use super::*;

        #[test]
        fn implement_pkcs7_padding() {
            let slice = b"YELLOW SUBMARINE";
            let expected_padding = b"YELLOW SUBMARINE\x04\x04\x04\x04";

            let blocks = blocks::Blocks::with_padding_from(20, slice);
            let actual_padding: Vec<u8> = blocks.into_iter().flatten().collect();

            assert_eq!(expected_padding.to_vec(), actual_padding);
        }
    }

    mod challenge10 {
        use super::*;

        #[test]
        fn _cbc_mode_decrypt() {
            let data = base64::decode(&_file_reader("../challenge-data/10.txt"));
            let key = b"YELLOW SUBMARINE";
            let iv = vec![b'0'; 16];
            
            let secret = cipher::cbc_mode_decrypt(&data, key, &iv);
            let message = cipher::cbc_mode_encrypt(&secret, key, &iv);
            assert_eq!(data.to_vec(), message)
        }
    }
}

fn _file_reader(path: &str) -> Vec<u8> {
    use std::io::BufRead;
    let file = std::fs::File::open(path).unwrap();

    std::io::BufReader::new(file)
        .lines()
        .flat_map(|result| result.expect("Failed to read line.").as_bytes().to_owned())
        .collect()
}