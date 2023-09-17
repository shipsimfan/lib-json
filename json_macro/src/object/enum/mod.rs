use super::Stream;
use proc_macro::{Delimiter, Ident};

mod member;

pub(crate) use member::EnumMember;

pub(crate) struct Enum {
    ident: Ident,
    members: Vec<EnumMember>,
}

impl Enum {
    pub(super) fn parse(mut stream: Stream) -> Self {
        let ident = stream
            .next_ident()
            .map_err(|_| "expected an ident")
            .unwrap();

        let mut body = stream
            .next_group(Delimiter::Brace)
            .map_err(|_| "expected an enum body")
            .unwrap();

        let mut members = Vec::new();
        while body.peek().is_some() {
            members.push(EnumMember::parse(&mut body));

            if body.take_punct(',', None).is_none() {
                break;
            }
        }

        if body.peek().is_some() {
            panic!("expected a comma");
        }

        Enum { ident, members }
    }
}
