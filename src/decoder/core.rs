use super::DecodeError;

pub fn decode_four_bytes(input: &[u8; 4]) -> Result<[u8; 3], DecodeError> {
    use crate::consts::CONVERT_TABLE;

    let mut decode_group = [0u8; 4];
    for (b, c) in decode_group.iter_mut().zip(input) {
        *b = CONVERT_TABLE
            .iter()
            .position(|&v| v == *c)
            .ok_or(DecodeError::InvalidValue(*c))? as u8;
    }

    let output = [
        (decode_group[0] << 2) + (decode_group[1] >> 4),
        (decode_group[1] << 4) + (decode_group[2] >> 2),
        (decode_group[2] << 6) + decode_group[3],
    ];

    Ok(output)
}
