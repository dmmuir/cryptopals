use std::collections::HashMap;
use std::hash::Hash;

pub fn byte_frequency(slice: &[u8]) -> HashMap<&u8, i32> {
    occurrence_count(slice)
}

pub fn contain_duplicates(slice: &[Vec<u8>]) -> bool {
    occurrence_count(slice)
        .iter()
        .any(|(_key, value)| *value > 1)
}

fn occurrence_count<T>(slice: &[T]) -> HashMap<&T, i32>
where
    T: Hash + Eq,
{
    let mut hit_records = HashMap::new();

    slice.into_iter().for_each(|item| {
        let counter = hit_records.entry(item).or_insert(0);
        *counter += 1;
    });

    hit_records
}

pub fn weights() -> impl Fn(HashMap<&u8, i32>) -> i32 {
    let weights: HashMap<u8, i32> = vec![
        (b'a', 108_167),
        (b'b', 101_482),
        (b'c', 102_782),
        (b'd', 104_253),
        (b'e', 112_702),
        (b'f', 102_228),
        (b'g', 102_015),
        (b'h', 106_094),
        (b'i', 106_094),
        (b'j', 100_153),
        (b'k', 100_772),
        (b'l', 104_025),
        (b'm', 102_406),
        (b'n', 106_749),
        (b'o', 107_507),
        (b'p', 101_929),
        (b'q', 100_095),
        (b'r', 105_987),
        (b's', 106_327),
        (b't', 109_056),
        (b'u', 102_758),
        (b'v', 100_978),
        (b'w', 102_360),
        (b'x', 100_150),
        (b'y', 101_974),
        (b'z', 100_074),
        (b' ', 113_000),
    ]
    .into_iter()
    .collect();

    move |hit_records| {
        hit_records
            .iter()
            .map(|(byte, hits)| *weights.get(&byte.to_ascii_lowercase()).unwrap_or(&1) * hits)
            .sum()
    }
}

pub fn find_key_size_score(range: std::ops::Range<usize>) -> impl Fn(&[u8]) -> Vec<(usize, u32)> {
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
        .map(|(a, b)| (a ^ b).count_ones())
        .sum()
}

fn normalize_distance(slice: &[u8], key_size: usize) -> u32 {
    let blocks: Vec<&[u8]> = slice.chunks(key_size).collect();

    let nd: Vec<u32> = blocks
        .windows(2)
        .map(|chunks| {
            if chunks[0].len() == chunks[1].len() {
                Some(1000 * hamm_distance(chunks[0], chunks[1]) / key_size as u32)
            } else {
                None
            }
        })
        .filter_map(|norm_distance| norm_distance)
        .collect();

    let divisor = nd.len() as u32;
    nd.iter().sum::<u32>() / divisor
}

#[cfg(test)]
mod test {
    use super::super::xor;
    use super::*;

    #[test]
    fn _weight_scores() {
        fn slice_from(tuple: (&u8, i32)) -> HashMap<&u8, i32> {
            vec![tuple].into_iter().collect()
        };
        let weight_scores = weights();

        assert_eq!(108_167, weight_scores(slice_from((&b'a', 1))));
        assert_eq!(101_482, weight_scores(slice_from((&b'b', 1))));
        assert_eq!(102_782, weight_scores(slice_from((&b'c', 1))));
        assert_eq!(104_253, weight_scores(slice_from((&b'd', 1))));
        assert_eq!(112_702, weight_scores(slice_from((&b'e', 1))));
        assert_eq!(102_228, weight_scores(slice_from((&b'f', 1))));
        assert_eq!(102_015, weight_scores(slice_from((&b'g', 1))));
        assert_eq!(106_094, weight_scores(slice_from((&b'h', 1))));
        assert_eq!(106_094, weight_scores(slice_from((&b'i', 1))));
        assert_eq!(100_153, weight_scores(slice_from((&b'j', 1))));
        assert_eq!(100_772, weight_scores(slice_from((&b'k', 1))));
        assert_eq!(104_025, weight_scores(slice_from((&b'l', 1))));
        assert_eq!(102_406, weight_scores(slice_from((&b'm', 1))));
        assert_eq!(106_749, weight_scores(slice_from((&b'n', 1))));
        assert_eq!(107_507, weight_scores(slice_from((&b'o', 1))));
        assert_eq!(101_929, weight_scores(slice_from((&b'p', 1))));
        assert_eq!(100_095, weight_scores(slice_from((&b'q', 1))));
        assert_eq!(105_987, weight_scores(slice_from((&b'r', 1))));
        assert_eq!(106_327, weight_scores(slice_from((&b's', 1))));
        assert_eq!(109_056, weight_scores(slice_from((&b't', 1))));
        assert_eq!(102_758, weight_scores(slice_from((&b'u', 1))));
        assert_eq!(100_978, weight_scores(slice_from((&b'v', 1))));
        assert_eq!(102_360, weight_scores(slice_from((&b'w', 1))));
        assert_eq!(100_150, weight_scores(slice_from((&b'x', 1))));
        assert_eq!(101_974, weight_scores(slice_from((&b'y', 1))));
        assert_eq!(100_074, weight_scores(slice_from((&b'z', 1))));
        assert_eq!(113_000, weight_scores(slice_from((&b' ', 1))));
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
