use super::{expect_char, Stream, QUOTATION_MARK};
use crate::{Input, ParseError, Value};

const ESCAPE: char = '\x5C'; // \

pub(super) fn parse<I: Input>(
    stream: &mut Stream<I>,
) -> Result<Value<'static>, ParseError<I::Error>> {
    parse_inner(stream, true).map(|string| string.into())
}

pub(super) fn parse_inner<I: Input>(
    stream: &mut Stream<I>,
    quotation_mark: bool,
) -> Result<String, ParseError<I::Error>> {
    if !quotation_mark {
        expect_char!(stream, QUOTATION_MARK)?;
    }

    let mut string = String::new();
    loop {
        let position = stream.position();
        let c = stream.next_err()?;

        if c == QUOTATION_MARK as char {
            return Ok(string);
        }

        string.push(if unescaped(c as u32) {
            c
        } else if c == ESCAPE {
            escape(stream)?
        } else {
            return Err(ParseError::UnexpectedCharacter(c, position));
        });
    }
}

fn escape<I: Input>(stream: &mut Stream<I>) -> Result<char, ParseError<I::Error>> {
    let position = stream.position();
    let c = stream.next_err()?;
    match c {
        '\x22' | '\x5C' | '\x2F' | '\x62' | '\x66' | '\x6E' | '\x72' | '\x74' => Ok(c),
        '\x75' => hex_char(stream),
        _ => Err(ParseError::UnexpectedCharacter(c, position)),
    }
}

fn hex_char<I: Input>(stream: &mut Stream<I>) -> Result<char, ParseError<I::Error>> {
    let position = stream.position();
    let mut char = 0;
    for _ in 0..4 {
        let position = stream.position();
        let c = stream.next_err()?;
        match c.to_digit(16) {
            Some(digit) => char = (char << 8) | digit,
            None => return Err(ParseError::UnexpectedCharacter(c, position)),
        }
    }

    char::from_u32(char).ok_or(ParseError::InvalidUTF8(position))
}

fn unescaped(c: u32) -> bool {
    c == 0x20 || c == 0x21 || (c >= 0x23 && c <= 0x5B) || (c >= 0x5D && c <= 0x10FFFF)
}
