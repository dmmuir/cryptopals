mod lib;

use lib::base64;
use lib::hex;
use lib::xor;

fn main() {
    println!(
        "Set 1 - Challenge 1: {}", 
        String::from_utf8(hex::decode(b"49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d")).unwrap()
    );
    println!(
        "Set 1 - Challenge 2: {}",
        String::from_utf8(hex::decode(b"746865206b696420646f6e277420706c6179")).unwrap()
    );

    println!(
        "Set 1 - Challenge 3: {}",
        String::from_utf8(
            xor::decrypt_single_byte_xor(
                &hex::decode(
                    b"1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736",
                ) // decode hex code to byte slice
            ) // decrypt single byte xor
            .unwrap() // unwrap option
            .2 // read decrypted slice only
        )
        .unwrap(),
    );

    println!(
        "Set 1 - Challenge 4: {}",
        String::from_utf8(detect_single_character_xor("./challenge-data/4.txt").2)
            .unwrap()
            .trim(),
    );

    println!(
        "Set 1 - Challenge 5:\n{}",
        String::from_utf8(xor::repeating_key_xor(b"ICE")(&hex::decode(
            &[
                b"0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272"
                    .to_vec(),
                b"a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f"
                    .to_vec(),
            ]
            .concat()
        )))
        .unwrap()
    );

    println!(
        "Set 1 - Challenge 6: {}\n{}",
        String::from_utf8(find_key("./challenge-data/6.txt")).unwrap(),
        String::from_utf8(break_repeating_key_xor("./challenge-data/6.txt")).unwrap(),
    )
}

fn detect_single_character_xor(path: &str) -> (usize, u8, Vec<u8>) {
    let hex_decoder = |line: String| hex::decode(line.as_bytes());
    let lines = file_read(path, hex_decoder);

    xor::find_single_byte_xor_lines(&lines)
        .iter()
        .max_by_key(|(_index, (score, _key, _text))| *score)
        .map(|(index, (_score, key, text))| (*index, *key, text.to_vec()))
        .unwrap()
}

fn find_key(path: &str) -> Vec<u8> {
    let base64_decoder = |file: String| base64::decode(file.as_bytes());
    let message: Vec<u8> = file_read_string(path, base64_decoder);

    xor::find_key(&message)
}

fn break_repeating_key_xor(path: &str) -> Vec<u8> {
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
