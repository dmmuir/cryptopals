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

    mod encode {
        use super::*;

        #[test]
        fn lowercase() {
            let s = b"hello world";
            let hex = b"68656c6c6f20776f726c64".to_vec();

            assert_eq!(hex.len(), encode(s).len());
            assert_eq!(hex, encode(s));
        }

        #[test]
        fn mix_case() {
            let s = b"Hello World";
            let hex = b"48656c6c6f20576f726c64".to_vec();

            assert_eq!(hex.len(), encode(s).len());
            assert_eq!(hex, encode(s));
        }

        #[test]
        fn uppercase() {
            let s = b"HELLO WORLD";
            let hex = b"48454c4c4f20574f524c44".to_vec();

            assert_eq!(hex.len(), encode(s).len());
            assert_eq!(hex, encode(s))
        }
    }

    mod decode {
        use super::*;

        #[test]
        fn lowercase() {
            let hex = b"68656c6c6f20776f726c64";
            let s = b"hello world".to_vec();

            assert_eq!(s.len(), decode(hex).len());
            assert_eq!(s, decode(hex));
        }

        #[test]
        fn mix_case() {
            let hex = b"48656c6c6f20576f726c64";
            let s = b"Hello World".to_vec();

            assert_eq!(s.len(), decode(hex).len());
            assert_eq!(s, decode(hex));
        }

        #[test]
        fn uppercase() {
            let hex = b"48454c4c4f20574f524c44";
            let s = b"HELLO WORLD".to_vec();

            assert_eq!(s.len(), decode(hex).len());
            assert_eq!(s, decode(hex))
        }
    }
}