#![feature(proc_macro_quote)]

use object::{Enum, Object, Struct};
use proc_macro::TokenStream;

mod object;
mod to_json;

#[proc_macro_derive(ToJSON)]
pub fn to_json(input: TokenStream) -> TokenStream {
    for token in input.clone() {
        println!("{:?}", token);
    }

    let derive = Object::parse(input, "ToJSON");
    to_json::generate(derive)
}
