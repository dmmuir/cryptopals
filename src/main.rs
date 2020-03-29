mod lib;

use lib::hex;

fn main() {
    println!(
        "Set 1 - Challenge 1: {}", 
        String::from_utf8(hex::decode(b"49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d")).unwrap()
    )
    ;
    println!(
        "Set 1 - Challenge 2: {}",
        String::from_utf8(hex::decode(b"746865206b696420646f6e277420706c6179")).unwrap()
    );
}
