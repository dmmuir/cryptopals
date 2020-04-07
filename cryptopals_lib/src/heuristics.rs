use std::collections::HashMap;
use std::hash::Hash;

pub fn byte_frequency(slice: &[u8]) -> HashMap<&u8, f32> {
    occurance_count(slice)
}

pub fn contain_duplicates(slice: &[Vec<u8>]) -> bool {
    occurance_count(slice)
        .iter()
        .any(|(_key, value)| *value > 1.0)
}

fn occurance_count<T>(slice: &[T]) -> HashMap<&T, f32>
where
    T: Hash + Eq,
{
    let mut hit_records = HashMap::new();

    slice.into_iter().for_each(|item| {
        let counter = hit_records.entry(item).or_insert(0.0f32);
        *counter += 1.0;
    });

    hit_records
}

pub fn weights() -> impl Fn(HashMap<&u8, f32>) -> i32 {
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

pub fn find_key_size_score(range: std::ops::Range<usize>) -> impl Fn(&[u8]) -> Vec<(usize, u32)> {
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
        fn slice_from(tuple: (&u8, f32)) -> HashMap<&u8, f32> {
            vec![tuple].into_iter().collect()
        };
        let weight_scores = weights();

        assert_eq!(108i32, weight_scores(slice_from((&b'a', 1.0f32))));
        assert_eq!(101i32, weight_scores(slice_from((&b'b', 1.0f32))));
        assert_eq!(103i32, weight_scores(slice_from((&b'c', 1.0f32))));
        assert_eq!(104i32, weight_scores(slice_from((&b'd', 1.0f32))));
        assert_eq!(113i32, weight_scores(slice_from((&b'e', 1.0f32))));
        assert_eq!(102i32, weight_scores(slice_from((&b'f', 1.0f32))));
        assert_eq!(102i32, weight_scores(slice_from((&b'g', 1.0f32))));
        assert_eq!(106i32, weight_scores(slice_from((&b'h', 1.0f32))));
        assert_eq!(106i32, weight_scores(slice_from((&b'i', 1.0f32))));
        assert_eq!(100i32, weight_scores(slice_from((&b'j', 1.0f32))));
        assert_eq!(101i32, weight_scores(slice_from((&b'k', 1.0f32))));
        assert_eq!(104i32, weight_scores(slice_from((&b'l', 1.0f32))));
        assert_eq!(102i32, weight_scores(slice_from((&b'm', 1.0f32))));
        assert_eq!(107i32, weight_scores(slice_from((&b'n', 1.0f32))));
        assert_eq!(108i32, weight_scores(slice_from((&b'o', 1.0f32))));
        assert_eq!(102i32, weight_scores(slice_from((&b'p', 1.0f32))));
        assert_eq!(100i32, weight_scores(slice_from((&b'q', 1.0f32))));
        assert_eq!(106i32, weight_scores(slice_from((&b'r', 1.0f32))));
        assert_eq!(106i32, weight_scores(slice_from((&b's', 1.0f32))));
        assert_eq!(109i32, weight_scores(slice_from((&b't', 1.0f32))));
        assert_eq!(103i32, weight_scores(slice_from((&b'u', 1.0f32))));
        assert_eq!(101i32, weight_scores(slice_from((&b'v', 1.0f32))));
        assert_eq!(102i32, weight_scores(slice_from((&b'w', 1.0f32))));
        assert_eq!(100i32, weight_scores(slice_from((&b'x', 1.0f32))));
        assert_eq!(102i32, weight_scores(slice_from((&b'y', 1.0f32))));
        assert_eq!(100i32, weight_scores(slice_from((&b'z', 1.0f32))));
        assert_eq!(113i32, weight_scores(slice_from((&b' ', 1.0f32))));
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
