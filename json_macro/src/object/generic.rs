use super::{Lifetime, Path, Stream, Type};
use crate::Generator;
use proc_macro::{Ident, Spacing};

pub(crate) struct Generic {
    lifetimes: Vec<(Lifetime, Vec<Lifetime>)>,
    types: Vec<GenericType>,
}

struct GenericType {
    r#type: Type,
    lifetimes: Vec<Lifetime>,
    traits: Vec<Path>,
}

impl Generic {
    pub(super) fn parse(stream: &mut Stream) -> Option<Self> {
        stream.take_punct('<', None)?;

        let mut lifetimes = Vec::new();
        let mut r#continue = true;
        while let Some(lifetime) = Lifetime::parse(stream) {
            let sub_lifetimes = if stream.take_punct(':', None).is_some() {
                todo!("Parse generic lifetime qualifiers")
            } else {
                Vec::new()
            };

            lifetimes.push((lifetime, sub_lifetimes));

            if stream.take_punct(',', None).is_none() {
                r#continue = false;
                break;
            }
        }

        let mut types = Vec::new();
        if r#continue {
            loop {
                let r#type = Type::parse(stream);

                let (lifetimes, traits) = if stream.take_punct(':', None).is_some() {
                    todo!("Handle generic type qualifiers")
                } else {
                    (Vec::new(), Vec::new())
                };

                types.push(GenericType {
                    r#type,
                    lifetimes,
                    traits,
                });

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

    pub(crate) fn generate(&self, generator: &mut Generator) {
        todo!("Generate generics")
    }

    pub(crate) fn generate_without_qualifiers(&self, generator: &mut Generator) {
        todo!("Generate generics with traits")
    }
}
