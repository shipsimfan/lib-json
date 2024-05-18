//! JSON parsing library

#![deny(missing_docs)]
#![deny(rustdoc::private_intra_doc_links)]
#![deny(rustdoc::unescaped_backticks)]
#![deny(rustdoc::redundant_explicit_links)]
#![warn(rustdoc::broken_intra_doc_links)]
#![feature(associated_type_defaults)]

mod deserialize;
mod error;
mod serialize;

pub use deserialize::from_string;
pub use error::{Error, Result};
pub use serialize::{to_bytes, to_bytes_pretty, to_str, to_str_pretty, to_write, to_write_pretty};

pub use data_format;
