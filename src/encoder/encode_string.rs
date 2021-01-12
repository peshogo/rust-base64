use crate::encoder::encode_bytes::encode;

/// Encodes a `String` to a base64 bytes
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
pub fn encode_string(s: &str) -> Vec<u8> {
    encode(s.as_bytes())
}
