use base64::{decode, decode_string, DecodeError};

#[test]
fn decode_invalid_char() {
    let out = decode(b"????");
    assert_eq!(out, Err(DecodeError::InvalidValue(b'?')));
}

#[test]
fn decode_invalid_length() {
    let out = decode(b"a");
    assert_eq!(out, Err(DecodeError::InvalidLength))
}

#[test]
fn decode_invalid_utf8() {
    let out = decode_string(b"12345678");
    match out {
        Err(DecodeError::InvalidUtf8(_)) => {}
        Err(_) => panic!("different error! {:?}", out),
        _ => panic!("some how input was a valid UTF-8: {:?}", out),
    }
}
