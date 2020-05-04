use crate::cipher::{cbc_mode_encrypt, ecb_mode_encrypt};

pub fn encryption_oracle(data: &[u8]) -> Vec<u8> {
    use rand::rngs::ThreadRng;
    use rand::Rng;

    fn gen_pad(rng: &mut ThreadRng, pad_len: usize) -> Vec<u8> {
        vec![0; pad_len].iter().map(|_| rng.gen::<u8>()).collect()
    }

    fn gen_key(mut rng: &mut ThreadRng) -> Vec<u8> {
        let key_size = 16;
        gen_pad(&mut rng, key_size)
    }

    fn pad_text(mut rng: &mut ThreadRng, data: &[u8]) -> Vec<u8> {
        let preppend_length = rng.gen_range(5, 10);
        let append_length = rng.gen_range(5, 10);

        [
            gen_pad(&mut rng, preppend_length),
            data.to_vec(),
            gen_pad(&mut rng, append_length),
        ]
        .concat()
    }

    fn encrypt_ecb_mode(mut rng: &mut ThreadRng, data: &[u8]) -> Vec<u8> {
        let key = gen_key(&mut rng);
        ecb_mode_encrypt(data, &key)
    }

    fn encrypt_cbc_mode(rng: &mut ThreadRng, data: &[u8]) -> Vec<u8> {
        let mut rng = rng;
        let iv = vec![rng.gen::<u8>(); 16];
        let key = gen_key(&mut rng);

        cbc_mode_encrypt(data, &key, &iv)
    }

    let mut rng = rand::thread_rng();

    let data = pad_text(&mut rng, data);

    if rng.gen::<bool>() {
        encrypt_ecb_mode(&mut rng, &data)
    } else {
        encrypt_cbc_mode(&mut rng, &data)
    }
}

pub fn ecb_encryption_oracle_generator(secret_text: &[u8]) -> impl Fn(&[u8]) -> Vec<u8> {
    fn gen_pad(pad_len: usize) -> Vec<u8> {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        vec![0; pad_len].iter().map(|_| rng.gen::<u8>()).collect()
    }

    fn gen_key() -> Vec<u8> {
        let key_size = 16;
        gen_pad(key_size)
    }

    fn pad_text(secret_text: &[u8], data: &[u8]) -> Vec<u8> {
        [data, secret_text].concat()
    }

    fn encrypt_ecb_mode(key: &[u8], data: &[u8]) -> Vec<u8> {
        ecb_mode_encrypt(data, &key)
    }

    let secret_text = secret_text.to_owned();
    let key = gen_key();

    move |data| {
        let data = pad_text(&secret_text, data);
        encrypt_ecb_mode(&key, &data)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    mod oracle {
        use super::*;

        #[test]
        fn _detect_encryption_mode() {
            use crate::cipher::detect_encryption_mode;

            // Make sure encryption_oracle doesn't panic.
            let data = b"Figuring to decrypt ecb mode encryption with key and back again!Figuring to decrypt ecb mode encryption with key and back again!"; // 64 bytes long -> 4 blocks of 16 bytes

            let secret = encryption_oracle(data);
            println!(
                "Type: {:?}, Secret {:?}",
                detect_encryption_mode(&secret),
                secret
            );
        }
    }
}
