/// Decode base64 String to Vec\<u8\>
/// 
/// # Example
/// 
/// ```
/// use base64;
/// 
/// // vec![1,2,3,4,5] -> base64 -> "AQIDBAU="
/// 
/// let base64_vec = String::from("AQIDBAU=");
/// let decoded_vec = base64::decode(&base64_vec);
/// 
/// assert_eq!(vec![1,2,3,4,5], decoded_vec);
/// ```
pub fn decode(s: &String) -> Vec<u8> {
    use decoder::core::decode_four_bytes;

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

	output
}