use super::{whitespace, Stream, VALUE_SEPERATOR};
use crate::{Input, ParseError, Value};

const END_ARRAY: char = '\x5D'; // ] right square bracket

pub(super) fn parse<I: Input>(
    stream: &mut Stream<I>,
) -> Result<Value<'static>, ParseError<I::Error>> {
    whitespace::parse(stream)?;

    let mut values: Vec<Value<'static>> = Vec::new();

    values.push(match stream.peek_err()? {
        END_ARRAY => return Ok(values.into()),
        _ => super::parse(stream)?,
    });

    loop {
        whitespace::parse(stream)?;

        let position = stream.position();
        match stream.next_err()? {
            VALUE_SEPERATOR => {}
            END_ARRAY => break,
            c => return Err(ParseError::UnexpectedCharacter(c as char, position)),
        }

        whitespace::parse(stream)?;

        values.push(super::parse(stream)?);
    }

    Ok(values.into())
}
