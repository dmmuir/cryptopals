use std::vec::IntoIter;

pub struct Blocks {
    n_m: (usize, usize),
    padding_length: usize,
    slice: Vec<u8>,
    state: States,
}

enum States {
    ORIGINAL,
    TRANSPOSED,
}

impl Blocks {
    pub fn from(block_size: usize, slice: &[u8]) -> Self {
        let slice = slice.to_vec();
        let padding_length = calculate_padding_size(block_size, slice.len());
        let n_m = calculate_dimensions(block_size, slice.len());

        Self {
            slice,
            n_m,
            padding_length,
            state: States::ORIGINAL,
        }
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
        self.state = States::TRANSPOSED
    }

    pub fn into_iter(&self) -> IntoIter<Vec<u8>> {
        self.chunk_slice().into_iter()
    }

    pub fn chunk_slice(&self) -> Vec<Vec<u8>> {
        let (n, _) = self.n_m;
        self.slice
            .chunks(n)
            .into_iter()
            .enumerate()
            .map(|(index, chunk)| match self.state {
                States::ORIGINAL => chunk.to_owned(),
                States::TRANSPOSED => self.remove_padding(index, chunk),
            })
            .collect()
    }

    fn remove_padding(&self, index: usize, block: &[u8]) -> Vec<u8> {
        let mut block = block.to_owned();
        let (_row_len, row_count) = self.n_m;

        if self.padding_length != 0 {
            if index >= row_count - self.padding_length {
                block.pop();
            }
        }

        block
    }
}

fn calculate_padding_size(block_size: usize, length: usize) -> usize {
    let remainder = length % block_size;

    if remainder != 0 {
        return block_size - length % block_size;
    }

    0
}

fn calculate_dimensions(block_size: usize, length: usize) -> (usize, usize) {
    let n = block_size;
    let m = length / n + if length % n != 0 { 1 } else { 0 };

    (n, m)
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

        #[test]
        fn with_valid_255_value() {
            let a: Vec<u8> = vec![
                vec![01, 255, 03, 04], 
                vec![05, 006, 07, 08], 
                vec![09],
                ]
                .into_iter()
                .flatten()
                .collect();
                
                let mut a_blocks = Blocks::from(4, &a);
                a_blocks.transpose();
                
                let a_t = vec![
                    vec![001, 5, 9],
                    vec![255, 6],
                    vec![003, 7],
                    vec![004, 8]
            ];

            assert_eq!(a_t, a_blocks.chunk_slice());
        }
    }
}
