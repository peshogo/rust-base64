pub fn encode(s: String) -> String {

	let mut output = String::new();
	let mut cv = vec![0u8,0,0];

	for (i,c) in s.as_bytes().iter().enumerate() {

		cv[i % 3] = *c;

		if i % 3 == 2 {
			let base64_string = encode_three_bytes(&cv);
			output.push_str(&base64_string);
		}
	}

	let i = s.len() % 3;

	match i {
		0 => output,
		1 => {
			cv[1] = 0;
			cv[2] = 0;
			let base64_string = encode_three_bytes(&cv);

			output.push_str(&base64_string[..2]);
			output.push_str("==");

			output
		},
		2 => {
			cv[2] = 0;
			let base64_string = encode_three_bytes(&cv);

			output.push_str(&base64_string[..3]);
			output.push_str("=");

			output
		},
		_ => unreachable!()
	}
}

fn encode_three_bytes(cv: &Vec<u8>) -> String {
	use consts::CONV_TABLE;

	let conv_table = CONV_TABLE.chars().collect::<Vec<char>>();

	let mut output = String::new();

	let based_bins = [
		cv[0] >> 2,
		((cv[0] << 6) >> 2) + (cv[1] >> 4),
		((cv[1] << 4) >> 2) + (cv[2] >> 6),
		(cv[2] << 2) >> 2
	];

	// println!("bins {:?}", based_bins);

	let based_chars: Vec<char> = based_bins.iter()
		.map(|&bin|conv_table[bin as usize].to_owned())
		.collect();

	// println!("{:?}", based_chars);

	based_chars.iter().for_each(|c| {
		output.push(*c);
	});

	output
}