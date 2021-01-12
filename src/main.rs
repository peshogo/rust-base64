extern crate base64;

use base64::{decode, decode_string};
use base64::{encode, encode_string};

fn main() {
    let input_bytes = [1, 2, 3, 4, 5];

    println!("before           :{:?}", input_bytes);
    let encoded = encode(&input_bytes);
    println!("encoded          :{:X?}", encoded);
    println!(
        "encoded as string:{}",
        String::from_utf8(encoded.clone()).unwrap()
    );
    let decoded = decode(&encoded);
    println!("after   :{:?}", decoded);

    println!("-----------------------------");

    let input_str = "Hello, World!";

    println!("before  :{}", input_str);
    let encoded = encode_string(&input_str);
    println!("encoded :{:X?}", encoded);
    let decoded = decode_string(&encoded);
    println!("after   :{:?}", decoded);
}
