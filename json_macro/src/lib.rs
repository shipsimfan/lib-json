use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

mod deserialize;

#[proc_macro_derive(Deserialize)]
pub fn derive_deserialize(input: TokenStream) -> TokenStream {
    deserialize::derive(parse_macro_input!(input as DeriveInput)).into()
}
