use super::decode;
use super::DecodeError;

/// Decode base64 `&[u8]` to `String`
///
/// # Example
///
/// ```
/// use base64;
///
/// // "Hello, World!" -> base64 -> "SGVsbG8sIFdvcmxkIQ==";
///
/// let base64_hello = b"SGVsbG8sIFdvcmxkIQ==";
/// let normal_hello = base64::decode_string(base64_hello).unwrap();
///
/// assert_eq!("Hello, World!", normal_hello);
/// ```
pub fn decode_string(input: &[u8]) -> Result<String, DecodeError> {
    let decoded_vec = decode(input)?;
    let output = String::from_utf8(decoded_vec)?;
    Ok(output)
}
