use crate::{DeserializeError, DeserializeErrorKind};
use lct_streams::Position;

impl<'de> DeserializeError<'de> {
    /// Gets the kind of error this is
    pub fn kind(&self) -> &DeserializeErrorKind<'de> {
        &self.kind
    }

    /// Gets the position the error occurred at
    pub fn position(&self) -> Option<Position> {
        self.position
    }
}
