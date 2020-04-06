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
            b"Lorem Ipsum is simply dummy text of the printing and typesetting industry.".to_vec();
        let key = b"Hello World";
        let xor = repeating_key_xor(key);

        let actual = xor(&xor(&message));

        assert_eq!(message, actual)
    }

    #[test]
    fn _decrypt_single_byte_xor() {
        let message =
            b"Lorem Ipsum is simply dummy text of the printing and typesetting industry.".to_vec();
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
