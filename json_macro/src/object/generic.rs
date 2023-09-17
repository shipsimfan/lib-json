use super::{Lifetime, Stream, Type};
use proc_macro::{Ident, Spacing};

pub(crate) struct Generic {
    lifetimes: Vec<Lifetime>,
    types: Vec<Type>,
}

impl Generic {
    pub(super) fn parse(stream: &mut Stream) -> Option<Self> {
        stream.take_punct('<', None)?;

        let mut lifetimes = Vec::new();
        let mut r#continue = true;
        while let Some(lifetime) = Lifetime::parse(stream) {
            lifetimes.push(lifetime);

            if stream.take_punct(',', None).is_none() {
                r#continue = false;
                break;
            }
        }

        let mut types = Vec::new();
        if r#continue {
            loop {
                types.push(Type::parse(stream));

                if stream.take_punct(',', None).is_none() {
                    break;
                }
            }
        }

        stream
            .next_punct('>', None)
            .map_err(|token| panic!("expected a right angle bracket {:?}", token.unwrap()))
            .unwrap();

        Some(Generic { lifetimes, types })
    }
}
