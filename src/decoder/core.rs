use std::{ fmt, error };

pub fn decode_four_bytes(cv: &Vec<u8>) -> Vec<u8> {
	use crate::consts::CONV_TABLE;

	let conv_table = CONV_TABLE.bytes().collect::<Vec<u8>>();

	let bytes = cv.iter().map(|&c|{
			conv_table.iter().position(|&b|b == c)
				.unwrap_or(0) as u8
		})
		.collect::<Vec<u8>>();

	let output= vec![
		(bytes[0] << 2) + (bytes[1] >> 4),
		(bytes[1] << 4) + (bytes[2] >> 2),
		(bytes[2] << 6) + bytes[3]
	];

	output
}

#[derive(Debug)]
pub struct Base64InvalidLengthError {}

impl fmt::Display for Base64InvalidLengthError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "The input of the base64 length was invalid")
	}
}

impl error::Error for Base64InvalidLengthError {
	fn description(&self) -> &str {
		"The input of the base64 length was invalid"
	}

	fn cause(&self) -> Option<&error::Error> {
		None
	}
}