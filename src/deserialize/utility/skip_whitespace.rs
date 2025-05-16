use crate::DeserializeError;
use lct_streams::SliceByteCharStream;

/// Skips whitespace in the stream, leaving the next non-whitespace character on the stream
pub(crate) fn skip_whitespace<'de>(
    stream: &mut SliceByteCharStream,
) -> Result<(), DeserializeError<'de>> {
    while let Some(c) = stream
        .peek()
        .map_err(|error| DeserializeError::new(error, stream.position()))?
    {
        match c {
            ' ' | '\t' | '\n' | '\r' => {}
            _ => break,
        }
        stream.next().unwrap();
    }

    Ok(())
}
