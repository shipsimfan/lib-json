use crate::{DeserializeError, DeserializeErrorKind};
use data_format::Unexpected;
use lct_streams::Position;

impl<'de> DeserializeError<'de> {
    /// Creates a new [`DeserializeError`]
    pub(crate) fn new<K: Into<DeserializeErrorKind<'de>>>(kind: K, position: Position) -> Self {
        DeserializeError {
            kind: kind.into(),
            position: Some(position),
        }
    }

    /// Creates a new [`DeserializeError`] with [`DeserializeErrorKind::Unexpected`]
    pub(crate) fn unexpected<U: Into<Unexpected>>(
        unexpected: U,
        expected: &'static str,
        position: Position,
    ) -> Self {
        DeserializeError::new(
            DeserializeErrorKind::Unexpected {
                unexpected: unexpected.into(),
                expected,
            },
            position,
        )
    }
}
