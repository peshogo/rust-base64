use super::DecodeError;

pub fn decode_four_bytes(cv: &[u8; 4]) -> Result<[u8; 3], DecodeError> {
    use crate::consts::CONVERT_TABLE;

    let mut bytes = [0u8; 4];
    for (b, c) in bytes.iter_mut().zip(cv) {
        *b = CONVERT_TABLE
            .iter()
            .position(|&v| v == *c)
            .ok_or(DecodeError::InvalidValue(*b))? as u8;
    }

    let output = [
        (bytes[0] << 2) + (bytes[1] >> 4),
        (bytes[1] << 4) + (bytes[2] >> 2),
        (bytes[2] << 6) + bytes[3],
    ];

    Ok(output)
}
