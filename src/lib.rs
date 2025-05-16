//! JSON parsing library

#![deny(missing_docs)]
#![deny(rustdoc::private_intra_doc_links)]
#![deny(rustdoc::unescaped_backticks)]
#![deny(rustdoc::redundant_explicit_links)]
#![warn(rustdoc::broken_intra_doc_links)]
#![feature(associated_type_defaults)]

mod deserialize;
mod serialize;
mod value;

pub use deserialize::{from_bytes, from_str, DeserializeError, DeserializeErrorKind};
pub use serialize::{
    to_bytes, to_bytes_pretty, to_str, to_str_pretty, to_write, to_write_pretty, SerializeError,
};
pub use value::Value;

pub use data_format;

#[cfg(test)]
pub mod tests;
