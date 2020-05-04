extern crate cryptopals_lib as lib;

use lib::{base64, blocks, cipher, hex, oracle, xor};

fn main() {
    println!("Set 1 - Challenge 1: {}", hex_decode_secret());

    println!("Set 1 - Challenge 2: {}", hex_decode_secret_again());

    println!("Set 1 - Challenge 3: {}", _decrypt_single_byte_xor());

    println!("Set 1 - Challenge 4: {}", detect_single_character_xor());

    println!("Set 1 - Challenge 5:\n{}", _repeating_key_xor());

    println!("Set 1 - Challenge 6: {}", break_repeating_key_xor());

    println!("Set 1 - Challenge 7: {}", decrypt_aes_128_in_ecb_mode());

    println!("Set 1 - Challenge 8: {}", detect_ecb_mode_encryption());

    println!("Set 2 - Challenge 9: {}", pad_yellow_submarine());

    println!("Set 2 - Challenge 10: {}", cbc_mode_decryption());

    println!("Set 2 - Challenge 11: {}", detection_oracle());

    println!("Set 2 - Challenge 12: {}", simple_ecb_decryption());
}

fn hex_decode_secret() -> String {
    hex_decode(b"49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d")
}

fn hex_decode_secret_again() -> String {
    hex_decode(b"746865206b696420646f6e277420706c6179")
}

fn _decrypt_single_byte_xor() -> String {
    let bytes =
        &hex::decode(b"1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736"); // decode hex code to byte slice

    let secret = xor::decrypt_single_byte_xor(bytes); // read decrypted slice only
    String::from_utf8(secret).unwrap()
}

fn detect_single_character_xor() -> String {
    let path = "./challenge-data/4.txt";
    let hex_decoder = |line: String| hex::decode(line.as_bytes());
    let lines = file_read(path, hex_decoder);

    let (index, key) = xor::detect_single_byte_xor_line(&lines);
    let secret = xor::single_byte_xor(key)(&lines[index]);

    String::from_utf8(secret).unwrap().trim().to_string()
}

fn _repeating_key_xor() -> String {
    let hex = [
        b"0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272".to_vec(),
        b"a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f".to_vec(),
    ]
    .concat();

    let secret = xor::repeating_key_xor(b"ICE")(&hex::decode(&hex));
    String::from_utf8(secret).unwrap()
}

fn break_repeating_key_xor() -> String {
    format!(
        "{}\n{}",
        String::from_utf8(find_key("./challenge-data/6.txt")).unwrap(),
        String::from_utf8(break_repeating_key("./challenge-data/6.txt")).unwrap(),
    )
}

fn decrypt_aes_128_in_ecb_mode() -> String {
    let base64_decoder = |file: String| base64::decode(file.as_bytes());
    let data = file_read_string("./challenge-data/7.txt", base64_decoder);
    let key = b"YELLOW SUBMARINE";
    let message = cipher::ecb_mode_decrypt(&data, key);

    let secret = String::from_utf8(message).unwrap();
    format!("{}\n{}", "YELLOW SUBMARINE", secret)
}

fn detect_ecb_mode_encryption() -> String {
    let hex_decoder = |file: String| hex::decode(file.as_bytes());
    let lines = file_read("./challenge-data/8.txt", hex_decoder);
    let hits = cipher::detect_ecb_mode_encryption(&lines);
    if let Some((line_number, _line)) = hits.first() {
        return format!("{}", line_number + 1);
    }

    String::new()
}

fn pad_yellow_submarine() -> String {
    let slice = b"YELLOW SUBMARINE";
    let blocks = blocks::Blocks::with_padding_from(20, slice);
    let padded_bytes: Vec<u8> = blocks.into_iter().flatten().collect();

    String::from_utf8(padded_bytes).unwrap()
}

fn cbc_mode_decryption() -> String {
    let base64_decoder = |file: String| base64::decode(file.as_bytes());
    let data = file_read_string("./challenge-data/10.txt", base64_decoder);
    let key = b"YELLOW SUBMARINE";
    let iv = vec![b'0'; 16];

    let message = cipher::cbc_mode_decrypt(&data, key, &iv);

    String::from_utf8(message).unwrap()
}

fn detection_oracle() -> String {
    let data = b"Figuring to decrypt ecb mode encryption with key and back again!Figuring to decrypt ecb mode encryption with key and back again!"; // 64 bytes long -> 4 blocks of 16 bytes
    let secret = oracle::encryption_oracle(data);

    cipher::detect_encryption_mode(&secret).to_string()
}

fn simple_ecb_decryption() -> String {
    let data = b"Figuring to decrypt ecb mode encryption with key and back again!Figuring to decrypt ecb mode encryption with key and back again!";
    let unknown_string = crate::base64::decode(b"Um9sbGluJyBpbiBteSA1LjAKV2l0aCBteSByYWctdG9wIGRvd24gc28gbXkgaGFpciBjYW4gYmxvdwpUaGUgZ2lybGllcyBvbiBzdGFuZGJ5IHdhdmluZyBqdXN0IHRvIHNheSBoaQpEaWQgeW91IHN0b3A/IE5vLCBJIGp1c3QgZHJvdmUgYnkK");

    let oracle = oracle::ecb_encryption_oracle_generator(&unknown_string);
    let message = cipher::aes_128_ecb_decrypt(&oracle, &oracle(data));
    String::from_utf8(message).unwrap()
}

fn hex_decode(bytes: &[u8]) -> String {
    let secret = hex::decode(bytes);
    String::from_utf8(secret).unwrap()
}

fn find_key(path: &str) -> Vec<u8> {
    let base64_decoder = |file: String| base64::decode(file.as_bytes());
    let message: Vec<u8> = file_read_string(path, base64_decoder);

    xor::find_key(&message)
}

fn break_repeating_key(path: &str) -> Vec<u8> {
    let base64_decoder = |file: String| base64::decode(file.as_bytes());
    let message: Vec<u8> = file_read_string(path, base64_decoder);

    xor::decrypt_repeating_key_xor(&message)
}

fn file_read<F>(path: &str, decoder: F) -> Vec<Vec<u8>>
where
    F: Fn(String) -> Vec<u8>,
{
    use std::io::BufRead;

    let file = std::fs::File::open(path).unwrap();
    std::io::BufReader::new(file)
        .lines()
        .filter_map(|result| result.ok())
        .map(decoder)
        .collect()
}

fn file_read_string<F>(path: &str, decoder: F) -> Vec<u8>
where
    F: Fn(String) -> Vec<u8>,
{
    use std::io::BufRead;

    let file = std::fs::File::open(path).unwrap();
    let file: String = std::io::BufReader::new(file)
        .lines()
        .filter_map(|result| result.ok())
        .collect();

    decoder(file)
}
