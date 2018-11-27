use std::error::Error;

/// Decode base64 String to normal String
/// 
/// # Example
/// 
/// ```
/// use base64;
/// 
/// // "Hello, World!" -> base64 -> "SGVsbG8sIFdvcmxkIQ==";
/// 
/// let base64_hello = String::from("SGVsbG8sIFdvcmxkIQ==");
/// let normal_hello = base64::decode_string(&base64_hello);
/// 
/// assert_eq!("Hello, World!", normal_hello);
/// ```
pub fn decode_string(s: &String) -> Result<String, Box<Error>> {
    use crate::decoder::decode;
    let decoded_vec = decode(s)?;
	let output = String::from_utf8(decoded_vec)?;
	Ok(output)
}
