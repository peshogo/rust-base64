#![warn(missing_docs)]

mod encoder;
mod decoder;
mod consts;

pub use encoder::{ encode, encode_string };
pub use decoder::{ decode, decode_string };
