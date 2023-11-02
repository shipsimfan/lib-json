#![feature(associated_type_defaults)]

mod error;
mod serialize;
mod value;

pub use error::{Error, Result};
pub use serialize::{to_bytes, to_bytes_pretty, to_str, to_str_pretty, to_write, to_write_pretty};
pub use value::{Array, Object, String, Value};
