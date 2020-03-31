mod lib;

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
        String::from_utf8(detect_single_character_xor("./challenge-data/4.txt").2).unwrap().trim(),
    );
}

fn detect_single_character_xor(path: &str) -> (usize, u8, Vec<u8>) {
    use std::io::BufRead;

    let f = std::fs::File::open(path).unwrap();
    let lines: Vec<Vec<u8>> = std::io::BufReader::new(f)
        .lines()
        .filter_map(|result| result.ok())
        .map(|line| hex::decode(line.as_bytes()))
        .collect();

    xor::find_single_byte_xor_lines(&lines)
        .iter()
        .max_by_key(|(_index, (score, _key, _text))| *score)
        .map(|(index, (_score, key, text))| (*index, *key, text.to_vec()))
        .unwrap()
}
