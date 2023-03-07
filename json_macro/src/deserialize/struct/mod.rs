use quote::{__private::TokenStream, quote};
use syn::{DataStruct, Generics, Ident};

mod field;

pub(super) fn derive(name: Ident, generics: Generics, struct_data: DataStruct) -> TokenStream {
    let fields = field::map_all(struct_data.fields);

    quote! {
        impl #generics json::Deserialize for #name #generics {
            fn deserialize(value: json::Value, key: Option<&str>) -> Result<Self, json::Error> {
                if !value.is_object() {
                    return Err(json::Error::InvalidType(key.map(|key| key.to_string()), json::Type::Object, json::Type::from_value(value)));
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
    }
}
