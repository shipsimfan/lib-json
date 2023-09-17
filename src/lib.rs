#![feature(associated_type_defaults)]

mod deserialize;
mod serialize;
mod value;

pub use deserialize::{deserialize, parse, DeserializeError, Input, ParseError, Position};
pub use json_macro::ToJSON;
pub use serialize::{serialize, ArrayIter, FormatterOutput, ObjectIter, Output, Serialize, ToJSON};
pub use value::{Array, Object, PrettyPrintable, PrettyPrinter, String, Value};

// TODO:
//  - Add ToJSON derive macro
//  - Add FromJSON derive macro

#[derive(ToJSON)]
pub(crate) struct Test1<'a> {
    name: std::string::String,
    value: usize,
    maybe: ::std::option::Option<Vec<usize>>,
    reference: &'a str,
    r#type: std::borrow::Cow<'a, str>,
}

#[derive(ToJSON)]
pub enum Test2 {
    Example1,
    Example2 = 2,
}
