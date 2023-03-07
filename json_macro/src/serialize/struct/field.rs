use crate::is_option;
use quote::{__private::TokenStream, quote};
use syn::{Field, Fields, Ident};

pub(super) fn map_all(fields: Fields) -> (Vec<TokenStream>, Vec<TokenStream>) {
    fields.into_iter().map(map).unzip()
}

fn map(field: Field) -> (TokenStream, TokenStream) {
    let name = field.ident.unwrap();
    let name_str = format!("{}", name);

    if is_option(&field.ty) {
        return map_option_field(name, name_str);
    }

    (
        quote! {
            output.insert(#name_str.to_string(), self.#name.serialize());
        },
        quote! {
            output.insert(#name_str.to_string(), self.#name.serialize_ref());
        },
    )
}

fn map_option_field(name: Ident, name_str: String) -> (TokenStream, TokenStream) {
    (
        quote! {
            self.#name.map(|item| output.insert(#name_str.to_string(), item.serialize()));
        },
        quote! {
            self.#name.as_ref().map(|item| output.insert(#name_str.to_string(), item.serialize_ref()));
        },
    )
}
