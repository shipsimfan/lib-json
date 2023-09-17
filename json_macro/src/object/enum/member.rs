use super::Stream;
use proc_macro::{Ident, Literal, Spacing};

pub(crate) struct EnumMember {
    ident: Ident,
    equals: Option<Literal>,
}

impl EnumMember {
    pub(super) fn parse(stream: &mut Stream) -> Self {
        let ident = stream
            .next_ident()
            .map_err(|_| panic!("expected an ident"))
            .unwrap();

        let equals = if stream.take_punct('=', Some(Spacing::Alone)).is_some() {
            Some(
                stream
                    .next_literal()
                    .map_err(|_| "expected an integer")
                    .unwrap(),
            )
        } else {
            None
        };

        EnumMember { ident, equals }
    }
}
