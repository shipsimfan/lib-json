#![feature(associated_type_defaults)]

mod deserialize;
mod serialize;
mod value;

pub use deserialize::{deserialize, parse, DeserializeError, Input, ParseError, Position};
pub use serialize::{serialize, ArrayIter, FormatterOutput, ObjectIter, Output, Serialize, ToJSON};
pub use value::{Array, Object, PrettyPrintable, PrettyPrinter, String, Value};

// TODO:
//  - Add deserialize
//  - Add ToJSON derive macro
//  - Add FromJSON derive macro
