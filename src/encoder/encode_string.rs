use crate::encoder::encode_bytes::encode;

/// Encodes a `String` to a base64 `Vec<u8>`
///
/// # Examples
///
/// ```
/// use base64;
///
/// let hello = "Hello, World!";
/// let base64_hello = base64::encode_string(&hello);
///
/// assert_eq!(b"SGVsbG8sIFdvcmxkIQ==", &base64_hello[..]);
/// ```
pub fn encode_string(input: &str) -> Vec<u8> {
    encode(input.as_bytes())
}
