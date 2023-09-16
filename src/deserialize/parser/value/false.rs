use super::{expect_char, Stream};
use crate::{Input, ParseError, Value};

pub(super) fn parse<I: Input>(
    stream: &mut Stream<I>,
) -> Result<Value<'static>, ParseError<I::Error>> {
    expect_char!(stream, '\x61')?;
    expect_char!(stream, '\x6C')?;
    expect_char!(stream, '\x73')?;
    expect_char!(stream, '\x65')?;
    Ok(false.into())
}
