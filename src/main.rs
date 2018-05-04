mod encoder;
mod decoder;

use encoder::encode;
use decoder::decode;

fn main() {
    let input = String::from("Hello 日本語!");
	let encoded = encode(input);
	println!("{}", &encoded);

	let decoded = decode(encoded);
	println!("{}", decoded);
}
