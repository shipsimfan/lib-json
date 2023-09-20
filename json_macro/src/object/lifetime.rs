use super::Stream;
use crate::Generator;
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

    pub(super) fn generate(&self, generator: &mut Generator) {
        generator.push_punct('\'', Spacing::Joint);
        generator.push_ident(self.ident.clone());
    }
}
