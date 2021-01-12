pub fn encode_three_bytes(cv: &[u8; 3]) -> [u8; 4] {
    use crate::consts::CONVERT_TABLE;

    let mut based_vals = [
        cv[0] >> 2,
        ((cv[0] << 6) >> 2) + (cv[1] >> 4),
        ((cv[1] << 4) >> 2) + (cv[2] >> 6),
        (cv[2] << 2) >> 2,
    ];

    for v in based_vals.iter_mut() {
        *v = CONVERT_TABLE[*v as usize];
    }

    based_vals
}
