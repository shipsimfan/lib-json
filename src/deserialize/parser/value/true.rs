use super::{expect_char, Stream};
use crate::{Input, ParseError, Value};

pub(super) fn parse<I: Input>(
    stream: &mut Stream<I>,
) -> Result<Value<'static>, ParseError<I::Error>> {
    expect_char!(stream, '\x72')?;
    expect_char!(stream, '\x75')?;
    expect_char!(stream, '\x65')?;
    Ok(true.into())
}
