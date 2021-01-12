use crate::encoder::core::encode_three_bytes;

/// Encode `Vec<u8>` to base64 bytes
///
/// # Example
///
/// ```
/// use base64;
///
/// let v = [1,2,3,4];
/// let base64_v = base64::encode(&v);
///
/// assert_eq!(b"AQIDBA==",&base64_v[..]);
/// ```
pub fn encode(input: &[u8]) -> Vec<u8> {
    let mut output = Vec::new();
    let mut encode_group = [0u8; 3];

    for (i, c) in input.iter().enumerate() {
        encode_group[i % 3] = *c;

        if i % 3 == 2 {
            let base64_vals = encode_three_bytes(&encode_group);
            output.extend_from_slice(&base64_vals);
        }
    }

    let i = input.len() % 3;

    match i {
        0 => output,
        1 => {
            encode_group[1] = 0;
            encode_group[2] = 0;
            let base64_string = encode_three_bytes(&encode_group);

            output.extend_from_slice(&base64_string[..2]);
            output.extend_from_slice(b"==");

            output
        }
        2 => {
            encode_group[2] = 0;
            let base64_string = encode_three_bytes(&encode_group);

            output.extend_from_slice(&base64_string[..3]);
            output.extend_from_slice(b"=");

            output
        }
        _ => unreachable!(),
    }
}
