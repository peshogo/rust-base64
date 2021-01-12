pub fn encode_three_bytes(input: &[u8; 3]) -> [u8; 4] {
    use crate::consts::CONVERT_TABLE;

    let mut base64_vals = [
        input[0] >> 2,
        ((input[0] << 6) >> 2) + (input[1] >> 4),
        ((input[1] << 4) >> 2) + (input[2] >> 6),
        (input[2] << 2) >> 2,
    ];

    for v in base64_vals.iter_mut() {
        *v = CONVERT_TABLE[*v as usize];
    }

    base64_vals
}
