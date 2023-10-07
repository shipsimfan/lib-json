use super::{Generic, Stream};
use proc_macro::{Delimiter, Ident};

mod member;

pub(crate) use member::StructMember;

pub(crate) struct Struct {
    ident: Ident,
    generic: Option<Generic>,
    members: Vec<StructMember>,
}

impl Struct {
    pub(super) fn parse(mut stream: Stream) -> Self {
        let ident = stream.next_ident().map_err(|_| "").unwrap();

        let generic = Generic::parse(&mut stream);

        let mut body = stream
            .next_group(Delimiter::Brace)
            .map_err(|_| "expected a struct body")
            .unwrap();

        let mut members = Vec::new();
        while body.peek().is_some() {
            members.push(StructMember::parse(&mut body));

            if body.take_punct(',', None).is_none() {
                break;
            }
        }

        if body.peek().is_some() {
            panic!("expected a comma");
        }

        Struct {
            ident,
            generic,
            members,
        }
    }

    pub(crate) fn ident(&self) -> &Ident {
        &self.ident
    }

    pub(crate) fn generic(&self) -> Option<&Generic> {
        self.generic.as_ref()
    }
}