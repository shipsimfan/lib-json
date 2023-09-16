use crate::{Input, ParseError, Value};
use stream::Stream;

mod stream;
mod value;
mod whitespace;

pub fn parse<I: Input>(input: &mut I) -> Result<Value<'static>, ParseError<I::Error>> {
    let mut stream = Stream::new(input)?;

    whitespace::parse(&mut stream)?;

    let value = value::parse(&mut stream)?;

    whitespace::parse(&mut stream)?;

    let position = stream.position();
    if let Some(c) = stream.next()? {
        Err(ParseError::UnexpectedCharacter(c as char, position))
    } else {
        Ok(value)
    }
}
