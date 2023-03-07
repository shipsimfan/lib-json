use proc_macro::TokenStream;
use quote::format_ident;
use syn::{parse_macro_input, DeriveInput, Type};

mod deserialize;
mod serialize;

#[proc_macro_derive(Deserialize)]
pub fn derive_deserialize(input: TokenStream) -> TokenStream {
    deserialize::derive(parse_macro_input!(input as DeriveInput)).into()
}

#[proc_macro_derive(Serialize)]
pub fn derive_serialize(input: TokenStream) -> TokenStream {
    serialize::derive(parse_macro_input!(input as DeriveInput)).into()
}

fn is_option(r#type: &Type) -> bool {
    let path = match r#type {
        Type::Path(path) => path,
        _ => return false,
    };

    if path.path.segments.len() != 1 {
        return false;
    }

    path.path.segments[0].ident == format_ident!("Option")
}
