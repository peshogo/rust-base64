pub fn encode_three_bytes(cv: &Vec<u8>) -> String {
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
