#![feature(associated_type_defaults)]

mod serialize;
mod value;

pub use serialize::{serialize, ArrayIter, FormatterOutput, ObjectIter, Output, Serialize, ToJSON};
pub use value::{Array, Object, PrettyPrintable, PrettyPrinter, String, Value};

// TODO:
//  - Add serialize_pretty
//  - Add modifying of values
//  - Add FromJSON & deserialize
