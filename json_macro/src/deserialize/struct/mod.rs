use quote::{__private::TokenStream, quote};
use syn::{DataStruct, Generics, Ident};

mod field;

pub(super) fn derive(name: Ident, generics: Generics, struct_data: DataStruct) -> TokenStream {
    let fields = field::map_all(struct_data.fields);

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
    }
}
