use encoder::core::encode_three_bytes;

/// Encode Vec\<u8\> to base64 String
/// 
/// # Example
/// 
/// ```
/// use base64;
/// 
/// let v = vec![1,2,3,4];
/// let base64_v = base64::encode(&v);
/// 
/// assert_eq!("AQIDBA==",base64_v);
/// ```
pub fn encode(v: &Vec<u8>) -> String {

	let mut output = String::new();
	let mut cv = vec![0u8,0,0];

	for (i,c) in v.iter().enumerate() {

		cv[i % 3] = *c;

		if i % 3 == 2 {
			let base64_string = encode_three_bytes(&cv);
			output.push_str(&base64_string);
		}
	}

	let i = v.len() % 3;

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

