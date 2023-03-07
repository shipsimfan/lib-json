use quote::{__private::TokenStream, format_ident, quote};
use syn::{Field, Fields, Ident, Type};

pub(super) fn map_all(fields: Fields) -> Vec<TokenStream> {
    fields.into_iter().map(map).collect()
}

fn map(field: Field) -> TokenStream {
    let name = field.ident.unwrap();
    let name_str = format!("{}", name);

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

fn map_option_field(name: Ident, name_str: String) -> TokenStream {
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
