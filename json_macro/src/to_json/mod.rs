use crate::Object;
use proc_macro::TokenStream;

mod r#enum;
mod r#struct;

pub(super) fn generate(object: Object) -> TokenStream {
    match object {
        Object::Enum(r#enum) => r#enum::generate(r#enum),
        Object::Struct(r#struct) => r#struct::generate(r#struct),
    }
}
