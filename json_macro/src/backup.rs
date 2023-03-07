use proc_macro::TokenStream;
use quote::{__private::TokenStream as TokenStream2, format_ident, quote};
use syn::{parse_macro_input, Data, DeriveInput, Field, Ident, Type};

#[proc_macro_derive(Deserialize)]
pub fn derive_deserialize(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let generics = input.generics;
    let name = input.ident;

    let fields: Vec<_> = match input.data {
        Data::Struct(struct_data) => struct_data.fields.into_iter().map(map_field).collect(),
        _ => panic!("Deserialize can only be derived for structs"),
    };

    quote! {
        impl #generics util::json::Deserialize for #name #generics {
            fn deserialize(value: util::json::Value, path: Option<&std::path::Path>, key: Option<&str>) -> Result<Self, util::json::Error> {
                if !value.is_object() {
                    return Err(util::json::Error::invalid_type(path, key, util::json::Type::Object, value));
                }

                let mut value = value.to_object().unwrap();
                let key_base = match key {
                    Some(key) => format!("{}.", key),
                    None => String::new(),
                };

                Ok(Self {
                    #(#fields),*
                })
            }
        }
    }.into()
}

fn map_field(field: Field) -> TokenStream2 {
    let name = field.ident.unwrap();
    let name_str = format!("{}", name);

    if is_option(&field.ty) {
        return map_option_field(name, name_str);
    }

    quote! {
        #name : match value.remove(#name_str) {
            Some(value) => util::json::deserialize(value, path, Some(&format!("{}{}", key_base, #name_str)))?,
            None => return Err(util::json::Error::missing_field(#name_str, path, key)),
        }
    }
}

fn map_option_field(name: Ident, name_str: String) -> TokenStream2 {
    quote! {
        #name : match value.remove(#name_str) {
            Some(value) => match value.is_null() {
                true => None,
                false => Some(util::json::deserialize(value, path, Some(&format!("{}{}", key_base, #name_str)))?),
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
