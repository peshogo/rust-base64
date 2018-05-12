#![warn(missing_docs)]

//! Base64
//! a library for encoding and decoding base64
//! 
//! works both for Strings and Vec\<u8\>!

mod encoder;
mod decoder;
mod consts;

pub use encoder::{ encode, encode_string };
pub use decoder::{ decode, decode_string };
