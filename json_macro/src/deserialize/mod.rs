mod r#struct;

use quote::__private::TokenStream;
use syn::{Data, DeriveInput};

pub(super) fn derive(input: DeriveInput) -> TokenStream {
    let generics = input.generics;
    let name = input.ident;

    match input.data {
        Data::Struct(struct_data) => r#struct::derive(name, generics, struct_data),
        _ => panic!("Deserialize can only be derived for structs"),
    }
}
