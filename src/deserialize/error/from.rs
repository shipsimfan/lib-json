use crate::{DeserializeError, DeserializeErrorKind};

impl<'de> From<DeserializeErrorKind<'de>> for DeserializeError<'de> {
    fn from(kind: DeserializeErrorKind<'de>) -> Self {
        DeserializeError {
            kind,
            position: None,
        }
    }
}
