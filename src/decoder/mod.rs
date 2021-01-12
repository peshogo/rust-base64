mod core;
mod decode_bytes;
mod decode_string;

use std::string::FromUtf8Error;

pub use self::decode_bytes::decode;
pub use self::decode_string::decode_string;

use thiserror::Error;

/// The Error type for Decoding base64
#[derive(Error, Debug, PartialEq)]
pub enum DecodeError {
    /// when there is a invalid value in base64
    #[error("There was an invalid base64 value {0} in input")]
    InvalidValue(u8),
    /// when the length is incorrect
    #[error("The length of the input was incorrect")]
    InvalidLength,
    /// when it can't be decoded into utf8
    #[error("The decoded bytes were invalid utf-8 characters")]
    InvalidUtf8(#[from] FromUtf8Error),
}
