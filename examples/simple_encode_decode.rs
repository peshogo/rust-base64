extern crate base64;

use base64::{decode, decode_string};
use base64::{encode, encode_string};

fn main() {
    let input_bytes = [1, 2, 3, 4, 5];

    println!("before  :{:?}", input_bytes);
    let encoded = encode(&input_bytes);
    println!("encoded :{:X?}", encoded);
    let decoded = decode(&encoded);
    println!("after   :{:?}", decoded);

    println!("-----------------------------");

    let s = "Hello, World!".to_owned();

    println!("before  :{}", s);
    let encoded = encode_string(&s);
    println!("encoded :{:X?}", encoded);
    let decoded = decode_string(&encoded);
    println!("after   :{:?}", decoded);
}
