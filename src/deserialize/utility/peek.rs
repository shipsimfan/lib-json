use crate::{DeserializeError, DeserializeErrorKind};
use lct_streams::{Position, SliceByteCharStream};

/// Get the next character from `stream`, without advancing it, returning an error if none is found
pub(crate) fn peek<'de>(
    stream: &mut SliceByteCharStream,
) -> Result<(char, Position), DeserializeError<'de>> {
    match stream.peek_pos() {
        Ok(Some(c)) => Ok(c),
        Ok(None) => Err(DeserializeError::new(
            DeserializeErrorKind::UnexpectedEndOfJSON,
            stream.position(),
        )),
        Err(error) => Err(DeserializeError::new(error, stream.position())),
    }
}
