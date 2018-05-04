pub fn decode(s: String) -> String {

	let mut output: Vec<u8> = Vec::new();
	let mut cv = vec![0u8;4];

	for (i, c) in s.as_bytes().iter().enumerate() {
		let i = i % 4;

		cv[i] = *c;

		if i % 4 == 3 {

			if cv[2] == '=' as u8 {
				cv[2] = 'A' as u8;
				cv[3] = 'A' as u8;

				let utf8_byte_slice = decode_four_bytes(&cv);
				utf8_byte_slice.iter().take(1).for_each(|&b|output.push(b));
			} else if cv[3] == '=' as u8 {
				cv[3] = 'A' as u8;

				let utf8_byte_slice = decode_four_bytes(&cv);
				utf8_byte_slice.iter().take(2).for_each(|&b|output.push(b));
			} else {
				let utf8_byte_slice = decode_four_bytes(&cv);
				utf8_byte_slice.iter().for_each(|&b|output.push(b));
			}

		}
	}

	let output = unsafe{ String::from_utf8_unchecked(output) };
	output
}

fn decode_four_bytes(cv: &Vec<u8>) -> Vec<u8> {
	let conv_table = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/".bytes().collect::<Vec<u8>>();

	let bytes = cv.iter().map(|&c|{
			conv_table.iter().position(|&b|{
					b == c
				})
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