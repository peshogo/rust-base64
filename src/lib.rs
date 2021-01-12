#![warn(missing_docs)]
#![forbid(unsafe_code)]

//! Base64
//! a library for encoding and decoding base64
//!
//! works both for Strings and Vec\<u8\>!

mod consts;
mod decoder;
mod encoder;

pub use crate::decoder::{decode, decode_string, DecodeError};
pub use crate::encoder::{encode, encode_string};
