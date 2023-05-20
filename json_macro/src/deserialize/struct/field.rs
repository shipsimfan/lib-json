use crate::is_option;
use quote::{__private::TokenStream, quote};
use syn::{Field, Fields, Ident};

pub(super) fn map_all(fields: Fields) -> Vec<TokenStream> {
    fields.into_iter().map(map).collect()
}

fn map(field: Field) -> TokenStream {
    let name = field.ident.unwrap();
    let name_str = format!("{}", name);

    let name_str = if name_str.starts_with("r#") {
        name_str.as_str()
    } else {
        &name_str[2..]
    };

    if is_option(&field.ty) {
        return map_option_field(name, name_str);
    }

    quote! {
        #name : match value.remove(#name_str) {
            Some(value) => json::deserialize(value, Some(&format!("{}{}", key_base, #name_str)))?,
            None => return Err(json::Error::MissingField(key.map(|key| key.to_string()), #name_str.to_string())),
        }
    }
}

fn map_option_field(name: Ident, name_str: &str) -> TokenStream {
    quote! {
        #name : match value.remove(#name_str) {
            Some(value) => match value.is_null() {
                true => None,
                false => Some(json::deserialize(value, Some(&format!("{}{}", key_base, #name_str)))?),
            }
            None => None,
        }
    }
}
