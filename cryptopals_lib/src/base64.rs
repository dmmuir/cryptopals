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

    mod encode {
        use super::*;

        #[test]
        fn foobar_tests() {
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
        }

        #[test]
        fn no_newline() {
            let base64_str = b"aGVsbG8gd29ybGQ=".to_vec();
            let s = b"hello world";

            assert_eq!(base64_str.len(), encode(s).len());
            assert_eq!(base64_str, encode(s));
        }

        #[test]
        fn with_newline() {
            let base64_str = b"aGVsbG8gd29ybGQK".to_vec();
            let s = b"hello world\n";

            assert_eq!(base64_str.len(), encode(s).len());
            assert_eq!(base64_str, encode(s));
        }

        #[test]
        fn large_file() {
            let encode_text =
                std::fs::read_to_string("../test-data/base64-test-encoded.txt").unwrap();
            let decode_text =
                std::fs::read_to_string("../test-data/base64-test-decoded.txt").unwrap();

            assert_eq!(
                encode_text.as_bytes().to_vec(),
                encode(decode_text.as_bytes())
            );
        }

        #[test]
        fn challenge_6_data() {
            extern crate base64 as cratesio_base64;

            let encoded_text = _file_reader("../challenge-data/6.txt");
            let decoded_text = cratesio_base64::decode(encoded_text.clone()).unwrap();

            assert_eq!(encoded_text.as_bytes().to_vec(), encode(&decoded_text),);
        }
    }

    mod decode {
        use super::*;

        #[test]
        fn foobar_tests() {
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
        }

        #[test]
        fn no_newline() {
            let s = b"hello world".to_vec();
            let base64_str = b"aGVsbG8gd29ybGQ=";

            assert_eq!(s.len(), decode(base64_str).len());
            assert_eq!(s, decode(base64_str));
        }

        #[test]
        fn with_newline() {
            let s = b"hello world\n".to_vec();
            let base64_str = b"aGVsbG8gd29ybGQK";

            assert_eq!(s.len(), decode(base64_str).len());
            assert_eq!(s, decode(base64_str));
        }
        #[test]
        fn large_file() {
            let encode_text =
                std::fs::read_to_string("../test-data/base64-test-encoded.txt").unwrap();
            let decode_text =
                std::fs::read_to_string("../test-data/base64-test-decoded.txt").unwrap();

            assert_eq!(
                decode_text.as_bytes().to_vec(),
                decode(encode_text.as_bytes()),
            );
        }

        #[test]
        fn challenge_6_data() {
            extern crate base64 as cratesio_base64;
            let encoded_text = _file_reader("../challenge-data/6.txt");

            assert_eq!(
                cratesio_base64::decode(encoded_text.clone()).unwrap(),
                decode(encoded_text.as_bytes()),
            );
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
