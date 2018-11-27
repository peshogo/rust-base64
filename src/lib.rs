#![warn(missing_docs)]

//! Base64
//! a library for encoding and decoding base64
//! 
//! works both for Strings and Vec\<u8\>!

mod encoder;
mod decoder;
mod consts;

pub use crate::encoder::{ encode, encode_string };
pub use crate::decoder::{ decode, decode_string };
