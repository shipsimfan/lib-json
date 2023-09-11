#![feature(associated_type_defaults)]

mod serialize;
mod value;

pub use serialize::{serialize, ArrayIter, ObjectIter, Serialize, ToJSON};
pub use value::{Array, Object, String, Value};
