use proc_macro::Spacing;

use super::{Generic, Keyword, Lifetime, Path, Stream};
use crate::Generator;

pub(crate) struct Type {
    lifetime: Option<Lifetime>,
    mutable: bool,
    path: Path,
    generic: Option<Generic>,
}

impl Type {
    pub(super) fn parse(stream: &mut Stream) -> Self {
        let (lifetime, mutable) = if stream.take_punct('&', None).is_some() {
            let lifetime = Lifetime::parse(stream)
                .ok_or("expected a lifetime")
                .unwrap();
            let mutable = stream.take_keyword(Keyword::Mut).is_some();
            (Some(lifetime), mutable)
        } else {
            (None, false)
        };

        let path = Path::parse(stream);
        let generic = Generic::parse(stream);

        Type {
            lifetime,
            mutable,
            path,
            generic,
        }
    }

    pub(super) fn generate(&self, generator: &mut Generator) {
        if let Some(lifetime) = &self.lifetime {
            generator.push_punct('&', Spacing::Joint);
            lifetime.generate(generator);

            if self.mutable {
                generator.push_keyword(Keyword::Mut);
            }
        }

        self.path.generate(generator);

        self.generic
            .as_ref()
            .map(|generic| generic.generate_without_qualifiers(generator));
    }
}
