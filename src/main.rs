mod encoder;
mod decoder;
mod consts;

use encoder::encode;
use decoder::decode;
use consts::CONV_TABLE;

fn main() {
    let input = String::from("Hello 日本語!");
	let encoded = encode(input);
	println!("{}", &encoded);

	let decoded = decode(encoded);
	println!("{}", decoded);

	println!("{}", CONV_TABLE);
}
