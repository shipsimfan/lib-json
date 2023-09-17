use super::Stream;
use proc_macro::{Ident, Spacing};

pub(crate) struct Lifetime {
    ident: Ident,
}

impl Lifetime {
    pub(super) fn parse(stream: &mut Stream) -> Option<Self> {
        stream.take_punct('\'', Some(Spacing::Joint))?;

        let ident = stream
            .next_ident()
            .map_err(|_| "expected an ident")
            .unwrap();

        Some(Lifetime { ident })
    }
}
