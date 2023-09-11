#![feature(associated_type_defaults)]

mod serialize;
mod value;

pub use serialize::{serialize, ArrayIter, ObjectIter, Serialize, ToJSON};
pub use value::{Array, Object, String, Value};

// TODO:
//  - Add pretty printing
//  - Add serialize_pretty
//  - Add modifying of values
//  - Add FromJSON & deserialize
