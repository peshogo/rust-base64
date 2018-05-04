/// This is the same as "simple_encode_decode" in the examples directory.
/// youcan run it with
/// 
/// ```bash
/// $ cargo run --example simple_encode_decode
/// ```

mod encoder;
mod decoder;
mod consts;

use encoder::{ encode_string, encode };
use decoder::{ decode_string, decode };

fn main() {
	let v = vec![1,2,3,4,5];
    
    println!("before  :{:?}", v);
    let encoded = encode(&v);
    println!("encoded :{}", encoded);
    let decoded = decode(&encoded);
    println!("after   :{:?}", decoded);

    println!();

    let s = String::from("Hello, World!");

    println!("before  :{}", s);
    let encoded = encode_string(&s);
    println!("encoded :{}", encoded);
    let decoded = decode_string(&encoded);
    println!("after   :{}", decoded);
}
