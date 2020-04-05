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
            .chunks_exact(2) // Issue: chunks_exact will drop the last byte if the hex string is odd number in length
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
    const BASE64_TABLE: &'static [u8] =
        b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

    #[allow(dead_code)]
    pub fn encode(bytes: &[u8]) -> Vec<u8> {
        fn base64(byte: u8) -> u8 {
            BASE64_TABLE[byte as usize]
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

        if bytes.is_empty() {
            return vec![];
        }
        let mod_table = [0, 2, 1];
        let pad_size = bytes.len() % 3;
        let buffer_size = 4 * (bytes.len() + mod_table[pad_size]) / 3;
        let mut buffer = vec![0; buffer_size];

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

        (0..mod_table[pad_size]).for_each(|i| {
            if let Some(b) = buffer.get_mut(buffer_size - 1 - i) {
                *b = b'=';
            }
        });

        buffer
    }

    #[allow(dead_code)]
    pub fn decode(bytes: &[u8]) -> Vec<u8> {
        #[rustfmt::skip]
        fn de_base64(byte: u8) -> u8 {
            // base64 -> decimal lookup table. 
            [//  0   1   2   3   4   5   6   7   8   9  10  11  12  13  14  15  16  17  18  19  20
                66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 64, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, //  1
                66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, //  2
                66, 62, 66, 66, 66, 63, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 66, 66, 66, 65, 66, //  3
                66, 66, 00, 01, 02, 03, 04, 05, 06, 07, 08, 09, 10, 11, 12, 13, 14, 15, 16, 17, 18, //  4
                19, 20, 21, 22, 23, 24, 25, 66, 66, 66, 66, 66, 66, 26, 27, 28, 29, 30, 31, 32, 33, //  5
                34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 66, 66, 66, //  6
                66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, //  7
                66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, //  8
                66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, //  9
                66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, // 10
                66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, // 11
                66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, // 12
                66, 66, 66, 66                                                                      // 13
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

        let mut buffer: Vec<u8> = padding_to_zero(bytes)
            .chunks(4)
            .flat_map(|b| {
                let n = pack_24_bit_bytes(b);

                vec![
                    ((n >> 16u8) & 255) as u8,
                    ((n >> 8) & 255) as u8,
                    (n & 255) as u8,
                ]
            })
            .collect();

        while let Some(byte) = buffer.pop() {
            if byte != 0 {
                buffer.push(byte);
                break;
            }
        }

        buffer
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn _encode() {
            assert_eq!(encode(b""), b"");
            assert_eq!(encode(b"f"), b"Zg==");
            assert_eq!(encode(b"fo"), b"Zm8=");
            assert_eq!(encode(b"foo"), b"Zm9v");
            assert_eq!(encode(b"foob"), b"Zm9vYg==");
            assert_eq!(encode(b"fooba"), b"Zm9vYmE=");
            assert_eq!(encode(b"foobar"), b"Zm9vYmFy");
            assert_eq!(encode(b"F"), b"Rg==");
            assert_eq!(encode(b"FO"), b"Rk8=");
            assert_eq!(encode(b"FOO"), b"Rk9P");
            assert_eq!(encode(b"FOOB"), b"Rk9PQg==");
            assert_eq!(encode(b"FOOBA"), b"Rk9PQkE=");
            assert_eq!(encode(b"FOOBAR"), b"Rk9PQkFS");

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

            let encode_text = std::fs::read_to_string("test-data/base64-test-encoded.txt").unwrap();
            let decode_text = std::fs::read_to_string("test-data/base64-test-decoded.txt").unwrap();

            assert_eq!(
                encode_text.as_bytes().to_vec(),
                encode(decode_text.as_bytes())
            );

            {
                extern crate base64 as cratesio_base64;

                let decoded_text =
                    cratesio_base64::decode(_file_reader("challenge-data/6.txt")).unwrap();

                assert_eq!(
                    _file_reader("challenge-data/6.txt").as_bytes().to_vec(),
                    encode(&decoded_text),
                );
            }
        }

        #[test]
        fn _decode() {
            assert_eq!(decode(b""), b"");
            assert_eq!(decode(b"Zg=="), b"f");
            assert_eq!(decode(b"Zm8="), b"fo");
            assert_eq!(decode(b"Zm9v"), b"foo");
            assert_eq!(decode(b"Zm9vYg=="), b"foob");
            assert_eq!(decode(b"Zm9vYmE="), b"fooba");
            assert_eq!(decode(b"Zm9vYmFy"), b"foobar");
            assert_eq!(decode(b"Rg=="), b"F");
            assert_eq!(decode(b"Rk8="), b"FO");
            assert_eq!(decode(b"Rk9P"), b"FOO");
            assert_eq!(decode(b"Rk9PQg=="), b"FOOB");
            assert_eq!(decode(b"Rk9PQkE="), b"FOOBA");
            assert_eq!(decode(b"Rk9PQkFS"), b"FOOBAR");

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

            let encode_text = std::fs::read_to_string("test-data/base64-test-encoded.txt").unwrap();
            let decode_text = std::fs::read_to_string("test-data/base64-test-decoded.txt").unwrap();

            assert_eq!(
                decode_text.as_bytes().to_vec(),
                decode(encode_text.as_bytes()),
            );

            {
                extern crate base64 as cratesio_base64;
                let encoded_text = _file_reader("challenge-data/6.txt");

                assert_eq!(
                    cratesio_base64::decode(encoded_text.clone()).unwrap(),
                    decode(encoded_text.as_bytes()),
                );
            }
        }
    }

    fn _file_reader(path: &str) -> String {
        use std::io::BufRead;
        let file = std::fs::File::open(path).unwrap();

        std::io::BufReader::new(file)
            .lines()
            .filter_map(|result| result.ok())
            .collect()
    }
}

pub mod xor {
    use super::blocks::Blocks;
    use super::heuristics::{byte_frequency, find_key_size_score, top_key, weights};

    #[allow(dead_code)]
    pub fn fixed_xor(a: &[u8], b: &[u8]) -> Vec<u8> {
        a.iter().zip(b.iter()).map(|(a, b)| a ^ b).collect()
    }

    pub fn single_byte_xor(key: u8) -> impl Fn(&[u8]) -> Vec<u8> {
        move |slice: &[u8]| slice.iter().map(|byte| byte ^ key).collect()
    }

    pub fn repeating_key_xor(key: &[u8]) -> impl Fn(&[u8]) -> Vec<u8> {
        let key = key.to_owned();
        move |slice: &[u8]| {
            key.iter()
                .cycle()
                .zip(slice.iter())
                .map(|(key, byte)| key ^ byte)
                .collect()
        }
    }

    pub fn decrypt_single_byte_xor(slice: &[u8]) -> Option<(i32, u8, Vec<u8>)> {
        let weight_scores = weights();
        (0..128) // Assuming the key is only within the ascii range
            .map(|key| {
                let decrypted_slice: Vec<u8> = single_byte_xor(key)(slice);
                let score = weight_scores(byte_frequency(&decrypted_slice));
                (score, key, decrypted_slice)
            })
            .max_by_key(|(score, _key, _slice)| *score)
    }

    pub fn decrypt_repeating_key_xor(slice: &[u8]) -> Vec<u8> {
        let key = find_key(slice);
        repeating_key_xor(&key)(slice)
    }

    pub fn find_single_byte_xor_lines(lines: &[Vec<u8>]) -> Vec<(usize, (i32, u8, Vec<u8>))> {
        lines
            .into_iter()
            .enumerate()
            .map(|(index, slice)| (index, decrypt_single_byte_xor(slice)))
            .filter(|(_index, option)| option.is_some())
            .map(|(index, option)| (index, option.unwrap()))
            .collect()
    }

    pub fn find_key(slice: &[u8]) -> Vec<u8> {
        let find_scores_for_range = find_key_size_score(2..41);
        let key_size = top_key(&find_scores_for_range(&slice));

        let mut blocks = Blocks::from(key_size, slice);
        blocks.transpose();

        blocks
            .chunk_into_iter()
            .map(|block| decrypt_single_byte_xor(&block).unwrap())
            .map(|(_score, key, _text)| key)
            .collect()
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

        #[test]
        fn _repeating_key_xor() {
            let message =
                b"Lorem Ipsum is simply dummy text of the printing and typesetting industry."
                    .to_vec();
            let key = b"Hello World";
            let xor = repeating_key_xor(key);

            let actual = xor(&xor(&message));

            assert_eq!(message, actual)
        }

        #[test]
        fn _decrypt_single_byte_xor() {
            let message =
                b"Lorem Ipsum is simply dummy text of the printing and typesetting industry."
                    .to_vec();
            let key_range = 0..128;

            for key in key_range {
                let message_copy = message.clone();
                let secret = single_byte_xor(key)(&message_copy);
                let (_score, found_key, actual) = decrypt_single_byte_xor(&secret).unwrap();

                assert_eq!(key, found_key,);

                assert_eq!(
                    String::from_utf8(message_copy).unwrap(),
                    String::from_utf8(actual).unwrap()
                );
            }
        }

        #[test]
        fn _find_key() {
            let key = b"Hello World".to_vec();
            let message = [
                [
                    b"Lorem Ipsum is simply dummy text of the ".to_vec(),
                    b"Sed ut perspiciatis unde omnis iste natu".to_vec(),
                ]
                .concat(),
                [
                    b"s error sit voluptatem accusantium dolor".to_vec(),
                    b"emque laudantium, totam rem aperiam, eaq".to_vec(),
                ]
                .concat(),
            ]
            .concat();

            let xor = repeating_key_xor(&key);
            let secret = xor(&message);
            assert_eq!(message, xor(&secret));

            let key_guess = find_key(&secret);
            assert_eq!(key, key_guess);
        }

        #[test]
        fn _decrypt_repeating_key_xor() {
            let key = b"Hello World";
            let message = [
                [
                    b"Lorem Ipsum is simply dummy text of the ".to_vec(),
                    b"Sed ut perspiciatis unde omnis iste natu".to_vec(),
                ]
                .concat(),
                [
                    b"s error sit voluptatem accusantium dolor".to_vec(),
                    b"emque laudantium, totam rem aperiam, eaq".to_vec(),
                ]
                .concat(),
            ]
            .concat();

            let xor = repeating_key_xor(key);
            let secret = xor(&message);
            assert_eq!(message, xor(&secret));

            let decrypt_attempt = decrypt_repeating_key_xor(&secret);
            assert_eq!(message, decrypt_attempt)
        }
    }
}

// TODO: Module needs a better name
pub mod heuristics {
    use std::collections::HashMap;

    pub fn byte_frequency(slice: &[u8]) -> HashMap<u8, f32> {
        let mut hit_records = HashMap::new();

        slice.iter().for_each(|b| {
            let counter = hit_records.entry(*b).or_insert(0.0);
            *counter += 1.0;
        });

        hit_records
    }

    pub fn weights() -> impl Fn(HashMap<u8, f32>) -> i32 {
        let weights: HashMap<u8, f32> = vec![
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
        .collect();

        move |hit_records| {
            hit_records
                .iter()
                .map(|(byte, hits)| {
                    *weights.get(&byte.to_ascii_lowercase()).unwrap_or(&1.00) * hits * 100.00
                })
                .sum::<f32>()
                .round() as i32
        }
    }

    pub fn find_key_size_score(
        range: std::ops::Range<usize>,
    ) -> impl Fn(&[u8]) -> Vec<(usize, u32)> {
        let normalize_distance = |slice: &[u8], key_size| -> u32 {
            let blocks_i: Vec<&[u8]> = slice.chunks(key_size).collect();
            let mut i = 0;
            blocks_i
                .windows(2)
                .map(|blocks| {
                    if blocks[0].len() == blocks[1].len() {
                        i += 2;
                        (1000 * hamm_distance(blocks[0], blocks[1])) / key_size as u32
                    } else {
                        0
                    }
                })
                .sum::<u32>()
                / i as u32
        };

        move |slice: &[u8]| {
            (range)
                .clone()
                .map(|key_size| (key_size, normalize_distance(&slice, key_size)))
                .collect()
        }
    }

    pub fn top_key(key_sizes: &[(usize, u32)]) -> usize {
        *top_n_keys(1, key_sizes)
            .first()
            .expect("No keys sizes determined")
    }

    pub fn top_n_keys(n: usize, key_sizes: &[(usize, u32)]) -> Vec<usize> {
        let mut key_sizes = key_sizes.to_owned();
        key_sizes.sort_by_key(|(_key_size, score)| *score);
        key_sizes[..n]
            .iter()
            .map(|(key_size, _score)| *key_size)
            .collect()
    }

    pub fn hamm_distance(a: &[u8], b: &[u8]) -> u32 {
        a.iter()
            .zip(b.iter())
            .fold(0u32, |d, (a, b)| d + (a ^ b).count_ones())
    }

    #[cfg(test)]
    mod test {
        use super::super::xor;
        use super::*;

        #[test]
        fn _weight_scores() {
            let weight_scores = weights();

            assert_eq!(
                108i32,
                weight_scores(vec![(b'a', 1.0f32)].into_iter().collect())
            );
            assert_eq!(
                101i32,
                weight_scores(vec![(b'b', 1.0f32)].into_iter().collect())
            );
            assert_eq!(
                103i32,
                weight_scores(vec![(b'c', 1.0f32)].into_iter().collect())
            );
            assert_eq!(
                104i32,
                weight_scores(vec![(b'd', 1.0f32)].into_iter().collect())
            );
            assert_eq!(
                113i32,
                weight_scores(vec![(b'e', 1.0f32)].into_iter().collect())
            );
            assert_eq!(
                102i32,
                weight_scores(vec![(b'f', 1.0f32)].into_iter().collect())
            );
            assert_eq!(
                102i32,
                weight_scores(vec![(b'g', 1.0f32)].into_iter().collect())
            );
            assert_eq!(
                106i32,
                weight_scores(vec![(b'h', 1.0f32)].into_iter().collect())
            );
            assert_eq!(
                106i32,
                weight_scores(vec![(b'i', 1.0f32)].into_iter().collect())
            );
            assert_eq!(
                100i32,
                weight_scores(vec![(b'j', 1.0f32)].into_iter().collect())
            );
            assert_eq!(
                101i32,
                weight_scores(vec![(b'k', 1.0f32)].into_iter().collect())
            );
            assert_eq!(
                104i32,
                weight_scores(vec![(b'l', 1.0f32)].into_iter().collect())
            );
            assert_eq!(
                102i32,
                weight_scores(vec![(b'm', 1.0f32)].into_iter().collect())
            );
            assert_eq!(
                107i32,
                weight_scores(vec![(b'n', 1.0f32)].into_iter().collect())
            );
            assert_eq!(
                108i32,
                weight_scores(vec![(b'o', 1.0f32)].into_iter().collect())
            );
            assert_eq!(
                102i32,
                weight_scores(vec![(b'p', 1.0f32)].into_iter().collect())
            );
            assert_eq!(
                100i32,
                weight_scores(vec![(b'q', 1.0f32)].into_iter().collect())
            );
            assert_eq!(
                106i32,
                weight_scores(vec![(b'r', 1.0f32)].into_iter().collect())
            );
            assert_eq!(
                106i32,
                weight_scores(vec![(b's', 1.0f32)].into_iter().collect())
            );
            assert_eq!(
                109i32,
                weight_scores(vec![(b't', 1.0f32)].into_iter().collect())
            );
            assert_eq!(
                103i32,
                weight_scores(vec![(b'u', 1.0f32)].into_iter().collect())
            );
            assert_eq!(
                101i32,
                weight_scores(vec![(b'v', 1.0f32)].into_iter().collect())
            );
            assert_eq!(
                102i32,
                weight_scores(vec![(b'w', 1.0f32)].into_iter().collect())
            );
            assert_eq!(
                100i32,
                weight_scores(vec![(b'x', 1.0f32)].into_iter().collect())
            );
            assert_eq!(
                102i32,
                weight_scores(vec![(b'y', 1.0f32)].into_iter().collect())
            );
            assert_eq!(
                100i32,
                weight_scores(vec![(b'z', 1.0f32)].into_iter().collect())
            );
            assert_eq!(
                113i32,
                weight_scores(vec![(b' ', 1.0f32)].into_iter().collect())
            );
        }

        #[test]
        fn _find_key_size_score() {
            let message = [
                [
                    b"Lorem Ipsum is simply dummy text of the ".to_vec(),
                    b"Sed ut perspiciatis unde omnis iste natu".to_vec(),
                ]
                .concat(),
                [
                    b"s error sit voluptatem accusantium dolor".to_vec(),
                    b"emque laudantium, totam rem aperiam, eaq".to_vec(),
                ]
                .concat(),
            ]
            .concat();

            let key_size_finder_test = |key: &[u8]| -> usize {
                let secret = xor::repeating_key_xor(key)(&message);
                let find_scores_for_range = find_key_size_score(2..20);

                top_key(&find_scores_for_range(&secret))
            };

            let key = b"Hello World";
            let key_sizes = key_size_finder_test(key);
            assert_eq!(key.len(), key_sizes);
        }

        #[test]
        fn _hamm_distance() {
            assert_eq!(37, hamm_distance(b"this is a test", b"wokka wokka!!!"));
            assert_eq!(2, hamm_distance(b"rover", b"river"));
            assert_eq!(2, hamm_distance(b"i", b"o"));
            assert_eq!(9, hamm_distance(b"karolin", b"kathrin"));
            assert_eq!(6, hamm_distance(b"karolin", b"kerstin"));
            assert_eq!(2, hamm_distance(&[4], &[8]));
            assert_eq!(3, hamm_distance(&[9], &[14]));
        }
    }
}

pub mod blocks {
    use std::vec::IntoIter;

    pub struct Blocks {
        n_m: (usize, usize),
        slice: Vec<u8>,
    }

    impl Blocks {
        pub fn from(block_size: usize, slice: &[u8]) -> Self {
            let slice = slice.to_vec();
            let n = block_size;
            let m = slice.len() / n + if slice.len() % n != 0 { 1 } else { 0 };

            Self { slice, n_m: (n, m) }
        }

        #[allow(dead_code)]
        pub fn transpose(&mut self) {
            let (n, m) = self.n_m;

            let mut new_blocks = vec![255; n * m];
            for i in 0..n * m {
                let x = i / m;
                let y = i % m;
                new_blocks[i] = *self.slice.get(n * y + x).unwrap_or(&255);
                // self.slice.swap(i, old_row); <--- TODO: figure out how to use `.swap(i, j)` instead.
            }
            self.slice = new_blocks;
            self.n_m = (m, n);
        }

        pub fn chunk_into_iter(&self) -> IntoIter<Vec<u8>> {
            self.chunk_slice().into_iter()
        }

        fn chunk_slice(&self) -> Vec<Vec<u8>> {
            let (n, _) = self.n_m;
            self.slice
                .chunks(n)
                .into_iter()
                .map(|chunk| {
                    chunk
                        .into_iter()
                        .filter(|b| b != &&255) //This could be a problem later. 255 should be a valid entry.
                        .map(|b| *b)
                        .collect()
                })
                .collect()
        }
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn _chunk_slice() {
            let a: Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
            let expected: Vec<Vec<u8>> = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];

            let actual = Blocks::from(3, &a).chunk_slice();

            assert_eq!(expected, actual);

            let a: Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
            let expected: Vec<Vec<u8>> = vec![vec![1, 2, 3, 4], vec![5, 6, 7, 8], vec![9]];

            let actual = Blocks::from(4, &a).chunk_slice();

            assert_eq!(expected, actual)
        }

        #[test]
        fn _into_iter() {
            let a: Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
            let expected: Vec<Vec<u8>> = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];

            let mut actual = Blocks::from(3, &a).chunk_into_iter();

            assert_eq!(Some(expected[0].to_owned()), actual.next());
            assert_eq!(Some(expected[1].to_owned()), actual.next());
            assert_eq!(Some(expected[2].to_owned()), actual.next());

            let a: Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
            let expected: Vec<Vec<u8>> = vec![vec![1, 2, 3, 4], vec![5, 6, 7, 8], vec![9]];

            let mut actual = Blocks::from(4, &a).chunk_into_iter();

            assert_eq!(Some(expected[0].to_owned()), actual.next());
            assert_eq!(Some(expected[1].to_owned()), actual.next());
            assert_eq!(Some(expected[2].to_owned()), actual.next());
        }

        #[rustfmt::skip]
        #[test]
        fn _transpose() {
            let a: Vec<u8> = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]
                .into_iter()
                .flatten()
                .collect();

            let mut a_blocks = Blocks::from(3, &a);
            a_blocks.transpose();

            let a_t = vec![vec![1, 4, 7], vec![2, 5, 8], vec![3, 6, 9]];

            assert_eq!(a_t, a_blocks.chunk_slice());

            let a: Vec<u8> = vec![
                vec![01, 02, 03, 04], 
                vec![05, 06, 07, 08], 
                vec![09, 10, 11, 12]]
                .into_iter()
                .flatten()
                .collect();

            let mut a_blocks = Blocks::from(4, &a);
            a_blocks.transpose();

            let a_t = vec![
                vec![1, 5, 9],
                vec![2, 6, 10],
                vec![3, 7, 11],
                vec![4, 8, 12]
            ];

            assert_eq!(a_t, a_blocks.chunk_slice());
            
            let a: Vec<u8> = vec![
                vec![1, 5, 9],
                vec![2, 6, 10],
                vec![3, 7, 11],
                vec![4, 8, 12]]
                .into_iter()
                .flatten()
                .collect();

            let mut a_blocks = Blocks::from(3, &a);
            a_blocks.transpose();

            let a_t = vec![
                vec![01, 02, 03, 04], 
                vec![05, 06, 07, 08], 
                vec![09, 10, 11, 12],
            ];

            assert_eq!(a_t, a_blocks.chunk_slice());
            let a: Vec<u8> = vec![
                vec![01, 02, 03, 04], 
                vec![05, 06, 07, 08], 
                vec![09, 10, 11],
            ]
            .into_iter()
            .flatten()
            .collect();
            
            let mut a_blocks = Blocks::from(4, &a);
            a_blocks.transpose();
            
            let a_t = vec![
                vec![1, 5, 09],
                vec![2, 6, 10],
                vec![3, 7, 11],
                vec![4, 8]
            ];

            assert_eq!(a_t, a_blocks.chunk_slice());

            let a: Vec<u8> = vec![
                vec![01, 02, 03, 04], 
                vec![05, 06, 07, 08], 
                vec![09, 10],
            ]
            .into_iter()
            .flatten()
            .collect();
            
            let mut a_blocks = Blocks::from(4, &a);
            a_blocks.transpose();
            
            let a_t = vec![
                vec![1, 5, 09],
                vec![2, 6, 10],
                vec![3, 7],
                vec![4, 8]];

            assert_eq!(a_t, a_blocks.chunk_slice());
            
            let a: Vec<u8> = vec![
                vec![01, 02, 03, 04], 
                vec![05, 06, 07, 08], 
                vec![09],
                ]
                .into_iter()
                .flatten()
                .collect();
                
                let mut a_blocks = Blocks::from(4, &a);
                a_blocks.transpose();
                
                let a_t = vec![
                    vec![1, 5, 9],
                    vec![2, 6],
                    vec![3, 7],
                    vec![4, 8]
            ];

            assert_eq!(a_t, a_blocks.chunk_slice());
        }
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
            let (_score, _key, actual) = xor::decrypt_single_byte_xor(&slice).unwrap();

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
            let (_index, (_score, _key, actual)) = &xor::find_single_byte_xor_lines(&[slice])[0];

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
            let message = base64::decode(&_file_reader("challenge-data/6.txt"));

            let expected_key = b"Terminator X: Bring the noise".to_vec();

            let key = xor::find_key(&message);
            assert_eq!(expected_key, key);
        }

        #[test]
        fn _decrypt_repeating_key_xor() {
            let message = base64::decode(&_file_reader("challenge-data/6.txt"));

            assert_eq!(
                std::fs::read_to_string("test-data/6-test.txt")
                    .unwrap()
                    .as_bytes()
                    .to_vec(),
                xor::decrypt_repeating_key_xor(&message),
            );
        }

        fn _file_reader(path: &str) -> Vec<u8> {
            use std::io::BufRead;
            let file = std::fs::File::open(path).unwrap();

            std::io::BufReader::new(file)
                .lines()
                .flat_map(|result| result.expect("Failed to read line.").as_bytes().to_owned())
                .collect()
        }
    }
}
