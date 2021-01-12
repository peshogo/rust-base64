use super::core::decode_four_bytes;
use super::DecodeError;

/// Decode base64 String to Vec\<u8\>
///
/// # Example
///
/// ```
/// use base64;
///
/// // [1,2,3,4,5] -> base64 -> "AQIDBAU="
///
/// let base64_vec = b"AQIDBAU=";
/// let decoded_vec = base64::decode(base64_vec).unwrap();
///
/// assert_eq!(&[1,2,3,4,5], &decoded_vec[..]);
/// ```
pub fn decode(s: &[u8]) -> Result<Vec<u8>, DecodeError> {
    let mut output: Vec<u8> = Vec::new();
    let mut cv = [0u8; 4];

    println!("{:?}", s.len());
    if s.len() % 4 != 0 {
        return Err(DecodeError::InvalidLength);
    }

    for (i, c) in s.iter().enumerate() {
        let i = i % 4;

        cv[i] = *c;

        if i % 4 == 3 {
            if cv[2] == b'=' {
                cv[2] = b'A';
                cv[3] = b'A';

                let utf8_byte_slice = decode_four_bytes(&cv)?;
                output.push(utf8_byte_slice[0]);
            } else if cv[3] == b'=' {
                cv[3] = b'A';

                let utf8_byte_slice = decode_four_bytes(&cv)?;
                output.extend_from_slice(&utf8_byte_slice[..2]);
            } else {
                let utf8_byte_slice = decode_four_bytes(&cv)?;
                output.extend_from_slice(&utf8_byte_slice);
            }
        }
    }

    Ok(output)
}
