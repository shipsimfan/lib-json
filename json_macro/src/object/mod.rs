use generic::Generic;
use keyword::Keyword;
use lifetime::Lifetime;
use macros::*;
use proc_macro::{Delimiter, TokenStream};
use stream::Stream;

mod r#enum;
mod generic;
mod keyword;
mod lifetime;
mod macros;
mod path;
mod stream;
mod r#struct;
mod r#type;

pub(super) use path::Path;
pub(super) use r#enum::Enum;
pub(super) use r#struct::{Struct, StructMember};
pub(super) use r#type::Type;

pub(super) enum Object {
    Struct(Struct),
    Enum(Enum),
}

impl Object {
    pub(super) fn parse(token_stream: TokenStream, trait_name: &str) -> Self {
        let mut stream = Stream::new(token_stream);

        // Parse visibility
        if stream.take_keyword(Keyword::Pub).is_some() {
            stream.take_group(Delimiter::Parenthesis);
        }

        // Get object type
        match stream
            .next_keyword("struct or enum")
            .map_err(|_| "expected struct or enum")
            .unwrap()
        {
            Keyword::Enum => Object::Enum(Enum::parse(stream)),
            Keyword::Struct => Object::Struct(Struct::parse(stream)),
            _ => panic!("'{}' may only be applied to structs and enums", trait_name),
        }
    }
}
