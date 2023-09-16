use super::{whitespace, Stream};
use crate::{Input, ParseError, Value};

mod array;
mod r#false;
mod null;
mod number;
mod object;
mod string;
mod r#true;

const VALUE_SEPERATOR: char = '\x2C'; // , comma
const BEGIN_ARRAY: char = '\x5B'; // [ left square bracket
const BEGIN_OBJECT: char = '\x7B'; // { left curly bracket
const MINUS: char = '\x2D'; // -
const QUOTATION_MARK: char = '\x22'; // "

pub(super) fn parse<I: Input>(
    stream: &mut Stream<I>,
) -> Result<Value<'static>, ParseError<I::Error>> {
    let position = stream.position();
    match stream.next_err()? {
        '\x66' => r#false::parse(stream),
        '\x6E' => r#null::parse(stream),
        '\x74' => r#true::parse(stream),

        BEGIN_ARRAY => array::parse(stream),
        BEGIN_OBJECT => object::parse(stream),

        MINUS => number::parse(stream, MINUS, position),
        x if x.is_ascii_digit() => number::parse(stream, x, position),

        QUOTATION_MARK => string::parse(stream),

        c => Err(ParseError::UnexpectedCharacter(c, position)),
    }
}
macro_rules! expect_char {
    ($stream: expr, $c: expr) => {{
        let position = $stream.position();
        match $stream.next_err() {
            Ok(c) => match $c == c {
                true => Ok(()),
                false => Err($crate::ParseError::UnexpectedCharacter(c as char, position)),
            },
            Err(error) => Err(error),
        }
    }};
}
use expect_char;
