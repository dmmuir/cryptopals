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

pub fn decrypt_single_byte_xor(slice: &[u8]) -> Vec<u8> {
    let key = find_single_byte_key(slice);
    single_byte_xor(key)(slice)
}

fn find_single_byte_key(slice: &[u8]) -> u8 {
    score_single_byte_keys_from(0..128)(slice)
        .into_iter()
        .max_by_key(|(score, _key)| *score)
        .map(|(_score, key)| key)
        .expect("Can't find key from empty slice.")
}

fn score_single_byte_keys_from(range: std::ops::Range<u8>) -> impl Fn(&[u8]) -> Vec<(i32, u8)> {
    let weight_scores = weights();

    move |slice| {
        (range.clone())
            .map(|key| {
                let decrypted_slice: Vec<u8> = single_byte_xor(key)(slice);
                let score = weight_scores(byte_frequency(&decrypted_slice));
                (score, key)
            })
            .collect()
    }
}

pub fn decrypt_repeating_key_xor(slice: &[u8]) -> Vec<u8> {
    let key = find_key(slice);
    repeating_key_xor(&key)(slice)
}

pub fn detect_single_byte_xor_line(lines: &[Vec<u8>]) -> (usize, u8) {
    find_single_byte_xor_lines(lines)
        .iter()
        .max_by_key(|(_index, score, _key)| *score)
        .map(|(index, _score, key)| (*index, *key))
        .unwrap()
}

fn find_single_byte_xor_lines(lines: &[Vec<u8>]) -> Vec<(usize, i32, u8)> {
    let score = score_single_byte_keys_from(0..128);
    let key_with_highest_score = |(index, scores): (usize, Vec<(i32, u8)>)| {
        let (score, key) = scores.into_iter().max_by_key(|(score, _)| *score).unwrap();
        (index, score, key)
    };

    lines
        .into_iter()
        .enumerate()
        .map(|(index, slice)| (index, score(slice)))
        .map(key_with_highest_score)
        .collect()
}

pub fn find_key(slice: &[u8]) -> Vec<u8> {
    let find_key_scores = find_key_size_score(2..41);
    let key_size = top_key(&find_key_scores(&slice));

    find_multi_byte_key(key_size, slice)
}

fn find_multi_byte_key(key_size: usize, slice: &[u8]) -> Vec<u8> {
    let mut blocks = Blocks::from(key_size, slice);
    blocks.transpose();

    blocks
        .into_iter()
        .map(|block| find_single_byte_key(&block))
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
        let message = b"Lorem Ipsum is simply dummy text of the printing and typesetting industry.";
        let key_range = 0..128;

        for key in key_range {
            let secret = single_byte_xor(key)(message);
            let actual = decrypt_single_byte_xor(&secret);

            assert_eq!(
                String::from_utf8(message.to_vec()).unwrap(),
                String::from_utf8(actual).unwrap()
            );
        }
    }

    #[test]
    fn _find_single_byte_xor() {
        let message = b"Lorem Ipsum is simply dummy text of the printing and typesetting industry.";
        let key_range = 0..128;

        for key in key_range {
            let secret = single_byte_xor(key)(message);
            let actual = find_single_byte_key(&secret);

            assert_eq!(key, actual);
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
