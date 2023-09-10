#![feature(associated_type_defaults)]

mod serialize;
mod value;

pub use serialize::{ArrayIter, ObjectIter, ToJSON};
pub use value::{Array, Object, String, Value};
