use super::{expect_char, string, whitespace, Stream, VALUE_SEPERATOR};
use crate::{Input, ParseError, String, Value};

const END_OBJECT: char = '\x7D'; // } right curly bracket
const NAME_SEPERATOR: char = '\x3A'; // : colon

pub(super) fn parse<I: Input>(
    stream: &mut Stream<I>,
) -> Result<Value<'static>, ParseError<I::Error>> {
    whitespace::parse(stream)?;

    let mut members: Vec<(String<'static>, Value<'static>)> = Vec::new();

    members.push(match stream.peek_err()? {
        END_OBJECT => return Ok(members.into()),
        _ => parse_member(stream)?,
    });

    loop {
        whitespace::parse(stream)?;

        let position = stream.position();
        match stream.next_err()? {
            VALUE_SEPERATOR => {}
            END_OBJECT => break,
            c => return Err(ParseError::UnexpectedCharacter(c as char, position)),
        }

        whitespace::parse(stream)?;

        members.push(parse_member(stream)?);
    }

    Ok(members.into())
}

pub(super) fn parse_member<I: Input>(
    stream: &mut Stream<I>,
) -> Result<(String<'static>, Value<'static>), ParseError<I::Error>> {
    let string = string::parse_inner(stream, false)?;

    whitespace::parse(stream)?;
    expect_char!(stream, NAME_SEPERATOR)?;
    whitespace::parse(stream)?;

    Ok((string.into(), super::parse(stream)?))
}
