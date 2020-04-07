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

    pub fn into_iter(&self) -> IntoIter<Vec<u8>> {
        self.chunk_slice().into_iter()
    }

    pub fn chunk_slice(&self) -> Vec<Vec<u8>> {
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

    mod chunk_slice {
        use super::*;

        #[test]
        fn three_x_three() {
            let a: Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
            let expected: Vec<Vec<u8>> = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];

            let actual = Blocks::from(3, &a).chunk_slice();

            assert_eq!(expected, actual);
        }

        #[test]
        fn four_x_three() {
            let a: Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
            let expected: Vec<Vec<u8>> = vec![vec![1, 2, 3, 4], vec![5, 6, 7, 8], vec![9]];

            let actual = Blocks::from(4, &a).chunk_slice();

            assert_eq!(expected, actual)
        }
    }

    mod into_iter {
        use super::*;

        #[test]
        fn three_x_three() {
            let a: Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
            let expected: Vec<Vec<u8>> = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];

            let mut actual = Blocks::from(3, &a).into_iter();

            assert_eq!(Some(expected[0].to_owned()), actual.next());
            assert_eq!(Some(expected[1].to_owned()), actual.next());
            assert_eq!(Some(expected[2].to_owned()), actual.next());
        }

        #[test]
        fn four_x_three() {
            let a: Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
            let expected: Vec<Vec<u8>> = vec![vec![1, 2, 3, 4], vec![5, 6, 7, 8], vec![9]];

            let mut actual = Blocks::from(4, &a).into_iter();

            assert_eq!(Some(expected[0].to_owned()), actual.next());
            assert_eq!(Some(expected[1].to_owned()), actual.next());
            assert_eq!(Some(expected[2].to_owned()), actual.next());
        }
    }

    #[rustfmt::skip]
    mod transpose {
        use super::*;

        #[test]
        fn three_x_three() {
            let a: Vec<u8> = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]
                .into_iter()
                .flatten()
                .collect();

            let mut a_blocks = Blocks::from(3, &a);
            a_blocks.transpose();

            let a_t = vec![vec![1, 4, 7], vec![2, 5, 8], vec![3, 6, 9]];

            assert_eq!(a_t, a_blocks.chunk_slice());
        }

        #[test]
        fn four_x_three() {
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
        }

        #[test]
        fn three_x_four() {
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
        }

        #[test]
        fn four_x_three_with_one_padding() {

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
        }

        #[test]
        fn four_x_three_with_two_padding() {
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
        }

        #[test]
        fn four_x_three_with_three_padding() {
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
