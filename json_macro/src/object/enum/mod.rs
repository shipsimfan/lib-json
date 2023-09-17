use super::Stream;
use proc_macro::{Delimiter, Ident};

pub(crate) struct Enum {
    ident: Ident,
}

impl Enum {
    pub(super) fn parse(mut stream: Stream) -> Self {
        let ident = stream.next_ident().map_err(|_| "expected an  e").unwrap();

        let mut body = stream
            .next_group(Delimiter::Brace)
            .map_err(|_| "expected an enum body")
            .unwrap();

        todo!("Parse enum body")
    }
}
