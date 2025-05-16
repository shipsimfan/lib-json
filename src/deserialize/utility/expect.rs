use crate::{DeserializeError, DeserializeErrorKind};
use lct_streams::{CharExpect, SliceByteCharStream};

/// Advance the stream validating it matches `predicate`
pub(crate) fn expect<'de, E: CharExpect>(
    stream: &mut SliceByteCharStream,
    expected: E,
    expected_display: &'static str,
) -> Result<(), DeserializeError<'de>> {
    for expected_c in expected.as_iter() {
        let (c, pos) = match stream.next_pos() {
            Ok(Some(c)) => c,
            Ok(None) => {
                return Err(DeserializeError::new(
                    DeserializeErrorKind::UnexpectedEndOfJSON,
                    stream.position(),
                ))
            }
            Err(error) => return Err(DeserializeError::new(error, stream.position())),
        };

        if c != expected_c {
            return Err(DeserializeError::new(
                DeserializeErrorKind::Unexpected {
                    unexpected: c.into(),
                    expected: expected_display,
                },
                pos,
            ));
        }
    }

    Ok(())
}
