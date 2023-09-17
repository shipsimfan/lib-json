use crate::object::{Stream, Type};
use proc_macro::{Ident, Spacing};

pub(crate) struct StructMember {
    name: Ident,
    r#type: Type,
}

impl StructMember {
    pub(super) fn parse(stream: &mut Stream) -> Self {
        let name = stream
            .next_ident()
            .map_err(|_| "expected an ident")
            .unwrap();

        stream
            .next_punct(':', Some(Spacing::Alone))
            .map_err(|_| "expected a colon")
            .unwrap();

        let r#type = Type::parse(stream);

        StructMember { name, r#type }
    }
}
