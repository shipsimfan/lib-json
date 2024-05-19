use std::borrow::Cow;

use super::Stream;
use crate::{Error, Result};

/// Deserializes a string from `stream`, converting it to valid UTF-8 if needed
pub(super) fn deserialize_string<'de>(stream: &mut Stream<'de>) -> Result<Cow<'de, str>> {
    stream.skip_whitespace();
    stream.expect(b'"', stream.index(), "a string")?;

    // Get the start index after moving over the beginning '"'
    let start_index = stream.index();
    let mut owned = None;

    while next_char(stream, start_index, &mut owned)? {}

    // Get the contained string before moving over the ending '"'
    let borrowed = stream.get_bytes(start_index);
    stream.next();

    Ok(match owned {
        Some(owned) => owned.into(),
        None => unsafe { std::str::from_utf8_unchecked(borrowed) }.into(),
    })
}

/// Creates a [`String`] from the previously parsed `stream`, starting at `start_index`, if one
/// hasn't been already. It then returns a mutable reference to the owned string for use.
fn create_owned<'a>(
    stream: &mut Stream,
    start_index: usize,
    owned: &'a mut Option<String>,
) -> &'a mut String {
    if owned.is_none() {
        *owned =
            Some(unsafe { String::from_utf8_unchecked(stream.get_bytes(start_index).to_owned()) });
    }

    owned.as_mut().unwrap()
}

/// Deserializes the next character from the stream
fn next_char(stream: &mut Stream, start_index: usize, owned: &mut Option<String>) -> Result<bool> {
    // Check if we have reached the end of the string. Using `peek` here to allow
    // `deserialize_string` to get the borrowed string without the final '"'
    let c = match stream.peek() {
        Some(b'"') => return Ok(false),
        Some(b'\\') => return escape(stream, start_index, owned).map(|_| true),
        Some(c) => {
            stream.next();
            c
        }
        None => return Err(Error::UnexpectedEndOfJSON),
    };

    if c > 0x7F {
        todo!("UTF-8 surrogate")
    } else if c < 0x20 {
        Err(Error::UnexpectedCharacter {
            unexpected: stream.get_bytes(start_index).to_owned(),
            expected: "a valid string",
        })
    } else {
        owned.as_mut().map(|owned| owned.push(c as char));
        Ok(())
    }
    .map(|_| true)
}

/// Deserializes the next character as a '\' then an escape character
fn escape(stream: &mut Stream, start_index: usize, owned: &mut Option<String>) -> Result<()> {
    let owned = create_owned(stream, start_index, owned);

    stream.next();

    match stream.next() {
        Some(b'"') => Ok(owned.push('"')),
        Some(b'\\') => Ok(owned.push('\\')),
        Some(b'/') => Ok(owned.push('/')),
        Some(b'b') => Ok(owned.push('\x08')),
        Some(b'f') => Ok(owned.push('\x0C')),
        Some(b'n') => Ok(owned.push('\n')),
        Some(b'r') => Ok(owned.push('\r')),
        Some(b't') => Ok(owned.push('\t')),
        Some(b'u') => unicode_escape(stream, start_index, owned),
        Some(_) => Err(Error::UnexpectedCharacter {
            unexpected: stream.get_bytes(start_index).to_owned(),
            expected: "a valid escape",
        }),
        None => Err(Error::UnexpectedEndOfJSON),
    }
}

/// Deserializes a unicode escape sequence from `stream` and places it in `owned`
fn unicode_escape(stream: &mut Stream, start_index: usize, owned: &mut String) -> Result<()> {
    let value = get_four_hex(stream, start_index)?;

    if value >= 0xD800 && value <= 0xDBFF {
        surrogate_pair_escape(value as u32, stream, start_index, owned)
    } else if value >= 0xDC00 && value <= 0xDFFF {
        Err(Error::UnexpectedCharacter {
            unexpected: stream.get_bytes(start_index).to_owned(),
            expected: "a valid string",
        })
    } else {
        owned.push(unsafe { char::from_u32_unchecked(value as u32) });
        Ok(())
    }
}

/// Deserializes a high surrogate pair from the stream
fn surrogate_pair_escape(
    high_surrogate: u32,
    stream: &mut Stream,
    start_index: usize,
    owned: &mut String,
) -> Result<()> {
    stream.expect(b'\\', start_index, "low UTF-16 surrogate")?;
    stream.expect(b'u', start_index, "low UTF-16 surrogate")?;

    let low_surrogate = get_four_hex(stream, stream.index())?;

    if low_surrogate < 0xDC00 || low_surrogate > 0xDFFF {
        return Err(Error::UnexpectedCharacter {
            unexpected: Vec::new(),
            expected: "",
        });
    }

    let c = 0x10000 + ((high_surrogate - 0xD800) * 0x400) + (low_surrogate as u32 - 0xDC00);
    owned.push(unsafe { char::from_u32_unchecked(c) });
    Ok(())
}

/// Deserializes 4 hex digits from `stream` into a [`u16`]
fn get_four_hex(stream: &mut Stream, start_index: usize) -> Result<u16> {
    let mut value = 0;

    for _ in 0..4 {
        let c = stream.next().ok_or(Error::UnexpectedEndOfJSON)?;

        value <<= 4;

        if c.is_ascii_digit() {
            value += (c - b'0') as u16;
        } else if c >= b'a' && c <= b'f' {
            value += (c - b'a' + 10) as u16;
        } else if c >= b'A' && c <= b'F' {
            value += (c - b'A' + 10) as u16;
        } else {
            return Err(Error::UnexpectedCharacter {
                unexpected: stream.get_bytes(start_index).to_owned(),
                expected: "4 hex digits",
            });
        }
    }

    Ok(value)
}
