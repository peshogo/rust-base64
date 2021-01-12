mod core;
mod decode_bytes;
mod decode_string;

use std::string::FromUtf8Error;

pub use self::decode_bytes::decode;
pub use self::decode_string::decode_string;

use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum DecodeError {
    #[error("There was an invalid base64 value {0} in input")]
    InvalidValue(u8),
    #[error("The length of the input was incorrect")]
    InvalidLength,
    #[error("The decoded bytes were invalid utf-8 characters")]
    InvalidUtf8(#[from] FromUtf8Error),
}
