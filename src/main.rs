extern crate base64;

use base64::{ encode_string, encode };
use base64::{ decode_string, decode };

fn main() {
	let v = vec![1,2,3,4,5];

	println!("before  :{:?}", v);
	let encoded = encode(&v);
	println!("encoded :{}", encoded);
	let decoded = decode(&encoded);
	println!("after   :{:?}", decoded);

	println!("-----------------------------");

	let s = "Hello, World!".into();

	println!("before  :{}", s);
	let encoded = encode_string(&s);
	println!("encoded :{}", encoded);
	let decoded = decode_string(&encoded);
	println!("after   :{:?}", decoded);
}
