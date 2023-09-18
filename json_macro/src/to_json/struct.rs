use crate::Struct;
use proc_macro::{quote, TokenStream};

pub(super) fn generate(r#struct: Struct) -> TokenStream {
    let mut output = generate_to_json(&r#struct);
    generate_into_value(&r#struct, &mut output);
    output
}

fn generate_to_json(r#struct: &Struct) -> TokenStream {
    todo!("Generate struct ToJSON")
}

fn generate_into_value(r#struct: &Struct, output: &mut TokenStream) {
    todo!("Generate struct Into<Value>")
}
