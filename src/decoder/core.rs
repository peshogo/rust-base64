pub fn decode_four_bytes(cv: &Vec<u8>) -> Vec<u8> {
	use consts::CONV_TABLE;

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