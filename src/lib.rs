pub mod hex {
    #[allow(dead_code)]
    pub fn encode(bytes: &[u8]) -> Vec<u8> {
        fn hex(byte: u8) -> u8 {
            b"0123456789abcdef"[byte as usize]
        }

        bytes
            .iter()
            .flat_map(|byte| vec![(hex((*byte >> 4) & 0xf)), (hex(*byte & 0xf))])
            .collect()
    }

    pub fn decode(bytes: &[u8]) -> Vec<u8> {
        fn de_hex(byte: u8) -> u8 {
            if byte > 57 {
                return byte + 10 - 97;
            }
            byte - 48
        }

        bytes
            .chunks_exact(2)
            .map(|b| {
                let b1 = de_hex(b[0]);
                let b2 = de_hex(b[1]);
                (b1 << 4) | b2
            })
            .collect()
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn _encode() {
            let s = b"hello world";
            let hex = b"68656c6c6f20776f726c64".to_vec();

            assert_eq!(hex.len(), encode(s).len());
            assert_eq!(hex, encode(s));

            let s = b"Hello World";
            let hex = b"48656c6c6f20576f726c64".to_vec();

            assert_eq!(hex.len(), encode(s).len());
            assert_eq!(hex, encode(s));

            let s = b"HELLO WORLD";
            let hex = b"48454c4c4f20574f524c44".to_vec();

            assert_eq!(hex.len(), encode(s).len());
            assert_eq!(hex, encode(s))
        }

        #[test]
        fn _decode() {
            let hex = b"68656c6c6f20776f726c64";
            let s = b"hello world".to_vec();

            assert_eq!(s.len(), decode(hex).len());
            assert_eq!(s, decode(hex));

            let hex = b"48656c6c6f20576f726c64";
            let s = b"Hello World".to_vec();

            assert_eq!(s.len(), decode(hex).len());
            assert_eq!(s, decode(hex));

            let hex = b"48454c4c4f20574f524c44";
            let s = b"HELLO WORLD".to_vec();

            assert_eq!(s.len(), decode(hex).len());
            assert_eq!(s, decode(hex))
        }
    }
}

pub mod base64 {

    #[allow(dead_code)]
    pub fn encode(bytes: &[u8]) -> Vec<u8> {
        fn base64(byte: u8) -> u8 {
            b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/"[byte as usize]
        }

        fn pack_24_bit_bytes(bytes: &[u8]) -> u32 {
            bytes
                .iter()
                .enumerate()
                .fold(0u32, |n, (i, b)| n + ((*b as u32) << (16 - (i * 8))))
        }

        fn split_6_bits_bytes_from(bytes: u32) -> [u8; 4] {
            let n0 = ((bytes >> 18) & 63) as u8;
            let n1 = ((bytes >> 12) & 63) as u8;
            let n2 = ((bytes >> 6) & 63) as u8;
            let n3 = (bytes & 63) as u8;

            [n0, n1, n2, n3]
        }

        let pad_size = bytes.len() % 3;
        let buffer_size = (bytes.len() * 4 / 3) + pad_size;
        let mut buffer = vec![b'='; buffer_size];

        bytes
            .chunks(3)
            .map(|byte_chunk| split_6_bits_bytes_from(pack_24_bit_bytes(byte_chunk)))
            .enumerate()
            .for_each(|(i, b)| {
                let x = i * 4;
                buffer[x] = base64(b[0]);

                if let Some(byte) = buffer.get_mut(x + 1) {
                    *byte = base64(b[1]);
                }
                if let Some(byte) = buffer.get_mut(x + 2) {
                    *byte = base64(b[2]);
                }
                if let Some(byte) = buffer.get_mut(x + 3) {
                    *byte = base64(b[3]);
                }
            });

        (0..pad_size).for_each(|i| {
            if let Some(b) = buffer.get_mut(buffer_size - i) {
                *b = b'=';
            }
        });

        buffer
    }

    #[allow(dead_code)]
    pub fn decode(bytes: &[u8]) -> Vec<u8> {
        fn de_base64(byte: u8) -> u8 {
            [
                66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 64, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66,
                66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66,
                66, 62, 66, 66, 66, 63, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 66, 66, 66, 65, 66,
                66, 66, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
                21, 22, 23, 24, 25, 66, 66, 66, 66, 66, 66, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35,
                36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 66, 66, 66, 66, 66,
                66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66,
                66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66,
                66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66,
                66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66,
                66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66,
                66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66,
                66, 66,
            ][byte as usize]
        }

        fn padding_to_zero(bytes: &[u8]) -> Vec<u8> {
            let mut bytes_mut = bytes.to_vec();
            let mut rev_bytes = bytes.iter().enumerate().rev();
            while let Some((i, b'=')) = rev_bytes.next() {
                if let Some(b) = bytes_mut.get_mut(i) {
                    *b = b'A';
                }
            }

            bytes_mut
        }

        fn pack_24_bit_bytes(byte_chunk: &[u8]) -> u32 {
            vec![
                (de_base64(byte_chunk[0]) as u32) << 18,
                (de_base64(byte_chunk[1]) as u32) << 12,
                (de_base64(byte_chunk[2]) as u32) << 6,
                de_base64(byte_chunk[3]) as u32,
            ]
            .into_iter()
            .sum()
        }

        let bytes = padding_to_zero(bytes);
        bytes
            .chunks(4)
            .flat_map(|b| {
                let n = pack_24_bit_bytes(b);

                vec![
                    ((n >> 16) & 255) as u8,
                    ((n >> 8) & 255) as u8,
                    (n & 255) as u8,
                ]
            })
            .filter(|b| *b != 0)
            .collect()
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn _encode() {
            let base64_str = b"aGVsbG8gd29ybGQ=".to_vec();
            let s = b"hello world";

            assert_eq!(base64_str.len(), encode(s).len());
            assert_eq!(base64_str, encode(s));

            let base64_str = b"aGVsbG8gd29ybGQK".to_vec();
            let s = b"hello world\n";

            assert_eq!(base64_str.len(), encode(s).len());
            assert_eq!(base64_str, encode(s));

            let base64_str = b"c2xka2ZqYXNkZmllaW9pbnNkZm9pbgo=".to_vec();
            let s = b"sldkfjasdfieioinsdfoin\n";

            assert_eq!(base64_str.len(), encode(s).len());
            assert_eq!(base64_str.to_vec(), encode(s));
        }

        #[test]
        fn _decode() {
            let s = b"hello world\n".to_vec();
            let base64_str = b"aGVsbG8gd29ybGQK";

            assert_eq!(s.len(), decode(base64_str).len());
            assert_eq!(s, decode(base64_str));
            assert_eq!(
                b"sldkfjasdfieioinsdfoin\n".len(),
                decode(b"c2xka2ZqYXNkZmllaW9pbnNkZm9pbgo=").len()
            );
            assert_eq!(
                b"sldkfjasdfieioinsdfoin\n",
                decode(b"c2xka2ZqYXNkZmllaW9pbnNkZm9pbgo=").as_slice()
            );
        }
    }
}

pub mod xor {
    #[allow(dead_code)]
    pub fn fixed_xor(a: &[u8], b: &[u8]) -> Vec<u8> {
        a.iter().zip(b.iter()).map(|(a, b)| a ^ b).collect()
    }

    #[allow(dead_code)]
    pub fn single_byte_xor(key: u8) -> impl Fn(&[u8]) -> Vec<u8> {
        move |slice: &[u8]| slice.iter().map(|byte| byte ^ key).collect()
    }

    #[allow(dead_code)]
    pub fn decrypt_single_byte_xor(slice: &[u8]) -> Option<(Vec<u8>, u8)> {
        use super::heuristics::byte_frequency;

        (0..127) // Assuming the key is only within the ascii range
            .map(|key| {
                let decrypted_slice: Vec<u8> = single_byte_xor(key)(slice);
                let score = byte_frequency(&decrypted_slice);

                (decrypted_slice, (key, score))
            })
            .max_by_key(|(_slice, (_key, score))| *score)
            .map(|(decrypted_slice, (key, _score))| (decrypted_slice, key))
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn _single_byte_xor() {
            let xor = single_byte_xor(b'a');
            let slice = b"Hello World";

            let actual = xor(&xor(slice));

            assert_eq!(slice.to_vec(), actual)
        }
    }
}

pub mod heuristics {
    use std::collections::HashMap;

    #[allow(dead_code)]
    pub fn byte_frequency(slice: &[u8]) -> i32 {
        let mut hit_records = HashMap::new();

        slice.iter().for_each(|b| {
            let counter = hit_records.entry(*b).or_insert(0.0);
            *counter += 1.0;
        });

        weight_scores(hit_records)
    }

    pub fn weight_scores(hit_records: HashMap<u8, f32>) -> i32 {
        let weights = vec![
            (b'a', 1.08167),
            (b'b', 1.01482),
            (b'c', 1.02782),
            (b'd', 1.04253),
            (b'e', 1.12702),
            (b'f', 1.02228),
            (b'g', 1.02015),
            (b'h', 1.06094),
            (b'i', 1.06094),
            (b'j', 1.00153),
            (b'k', 1.00772),
            (b'l', 1.04025),
            (b'm', 1.02406),
            (b'n', 1.06749),
            (b'o', 1.07507),
            (b'p', 1.01929),
            (b'q', 1.00095),
            (b'r', 1.05987),
            (b's', 1.06327),
            (b't', 1.09056),
            (b'u', 1.02758),
            (b'v', 1.00978),
            (b'w', 1.02360),
            (b'x', 1.00150),
            (b'y', 1.01974),
            (b'z', 1.00074),
            (b' ', 1.13000),
        ]
        .into_iter()
        .collect::<HashMap<u8, f32>>();

        hit_records
            .iter()
            .map(|(byte, hits)| *weights.get(&byte.to_ascii_lowercase()).unwrap_or(&1.00) * hits)
            .sum::<f32>() as i32
    }
}

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
            let (actual, _likely_byte) = xor::decrypt_single_byte_xor(&slice).unwrap();

            assert_eq!(
                String::from("Cooking MC's like a pound of bacon"),
                String::from_utf8(actual).unwrap()
            )
        }
    }
}
