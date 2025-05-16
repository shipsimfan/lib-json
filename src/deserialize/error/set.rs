use crate::DeserializeError;
use lct_streams::Position;

impl<'de> DeserializeError<'de> {
    /// Sets the position that the error occurred at, if one hasn't already been sets
    pub(crate) fn set_position(&mut self, position: Position) {
        if self.position.is_none() {
            self.position = Some(position);
        }
    }
}
