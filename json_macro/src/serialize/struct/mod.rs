use quote::{__private::TokenStream, quote};
use syn::{DataStruct, Generics, Ident};

mod field;

pub(super) fn derive(name: Ident, generics: Generics, struct_data: DataStruct) -> TokenStream {
    let (fields, fields_ref) = field::map_all(struct_data.fields);

    quote! {
        impl #generics json::Serialize for #name #generics {
            fn serialize_ref(&self) -> json::Value {
                use json::Serialize;

                let mut output = json::FxHashMap::default();

                #(#fields_ref)*

                json::Value::Object(output)
            }

            fn serialize(self) -> json::Value {
                use json::Serialize;

                let mut output = json::FxHashMap::default();

                #(#fields)*

                json::Value::Object(output)
            }
        }
    }
}
