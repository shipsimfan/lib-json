use super::Stream;
use crate::Generator;
use proc_macro::{Ident, Spacing};

pub(crate) struct Path {
    leading: bool,
    path: Vec<Ident>,
    r#final: Ident,
}

impl Path {
    pub(super) fn parse(stream: &mut Stream) -> Self {
        let leading = if stream.take_punct(':', Some(Spacing::Joint)).is_some() {
            stream
                .next_punct(':', Some(Spacing::Alone))
                .map_err(|_| "expected a colon")
                .unwrap();
            true
        } else {
            false
        };

        let mut r#final = stream
            .next_ident()
            .map_err(|token| panic!("expected an ident {:?}", token.unwrap()))
            .unwrap();

        let mut path = Vec::new();
        while stream.take_punct(':', Some(Spacing::Joint)).is_some() {
            stream
                .next_punct(':', Some(Spacing::Alone))
                .map_err(|_| "expected a colon")
                .unwrap();

            path.push(r#final);
            r#final = stream
                .next_ident()
                .map_err(|_| "expected an ident")
                .unwrap();
        }

        Path {
            leading,
            path,
            r#final,
        }
    }

    pub(super) fn generate(&self, generator: &mut Generator) {
        if self.leading {
            generator.push_punct(':', Spacing::Joint);
            generator.push_punct(':', Spacing::Alone);
        }

        for part in &self.path {
            generator.push_ident(part.clone());
            generator.push_punct(':', Spacing::Joint);
            generator.push_punct(':', Spacing::Alone);
        }

        generator.push_ident(self.r#final.clone());
    }
}