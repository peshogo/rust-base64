use encoder::encode_bytes::encode;

/// Encodes a String to a base64 String
/// 
/// # Examples
/// 
/// ```
/// use base64;
/// 
/// let hello = String::from("Hello, World!");
/// let base64_hello = base64::encode_string(&hello);
/// 
/// assert_eq!("SGVsbG8sIFdvcmxkIQ==", base64_hello);
/// ```
pub fn encode_string(s: &String) -> String {
	encode(&s.bytes().collect::<Vec<u8>>())
}
