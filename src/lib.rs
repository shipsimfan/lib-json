//! JSON parsing library

#![cfg_attr(feature = "no_std", no_std)]
#![deny(missing_docs)]
#![deny(rustdoc::private_intra_doc_links)]
#![deny(rustdoc::unescaped_backticks)]
#![deny(rustdoc::redundant_explicit_links)]
#![warn(rustdoc::broken_intra_doc_links)]

#[cfg(feature = "no_std")]
extern crate alloc;

mod deserialize;
mod serialize;
mod value;

pub use deserialize::{from_bytes, from_str, DeserializeError, DeserializeErrorKind};
#[cfg(not(feature = "no_std"))]
use serialize::{to_bytes, to_bytes_pretty, to_write, to_write_pretty};
pub use serialize::{to_str, to_str_pretty, SerializeError};
pub use value::Value;

pub use data_format;

#[cfg(test)]
pub mod tests;
