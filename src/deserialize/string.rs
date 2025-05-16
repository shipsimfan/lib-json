use crate::{
    deserialize::{expect, peek, skip_whitespace, Result},
    DeserializeError, DeserializeErrorKind,
};
use lct_streams::{Position, SliceByteCharStream};
use std::borrow::Cow;

/// Deserializes a string from `stream`, converting it to valid UTF-8 if needed
pub(super) fn deserialize_string<'de>(
    stream: &mut SliceByteCharStream<'de>,
) -> Result<'de, (Cow<'de, str>, Position)> {
    skip_whitespace(stream)?;
    expect(stream, '\"', "a string")?;

    // Get the start index after moving over the beginning '"'
    let position = stream.position();
    let start_offset = stream.offset();
    let mut owned = None;

    while next_char(stream, start_offset, &mut owned)? {}

    // Get the contained string before moving over the ending '"'
    let borrowed = &stream.bytes()[start_offset..stream.offset()];
    stream.next().unwrap();

    Ok((
        match owned {
            Some(owned) => owned.into(),
            None => unsafe { std::str::from_utf8_unchecked(borrowed) }.into(),
        },
        position,
    ))
}

/// Creates a [`String`] from the previously parsed `stream`, starting at `start_index`, if one
/// hasn't been already. It then returns a mutable reference to the owned string for use.
fn create_owned<'a>(
    stream: &mut SliceByteCharStream,
    start_offset: usize,
    owned: &'a mut Option<String>,
) -> &'a mut String {
    if owned.is_none() {
        *owned = Some(unsafe {
            String::from_utf8_unchecked(stream.bytes()[start_offset..stream.offset()].to_owned())
        });
    }

    owned.as_mut().unwrap()
}

/// Deserializes the next character from the stream
fn next_char<'de>(
    stream: &mut SliceByteCharStream<'de>,
    start_offset: usize,
    owned: &mut Option<String>,
) -> Result<'de, bool> {
    // Check if we have reached the end of the string. Using `peek` here to allow
    // `deserialize_string` to get the borrowed string without the final '"'
    let (c, pos) = match peek(stream)? {
        ('"', _) => return Ok(false),
        ('\\', _) => return escape(stream, start_offset, owned).map(|_| true),
        (c, pos) => {
            stream.next().unwrap();
            (c, pos)
        }
    };

    if c < ' ' {
        return Err(DeserializeError::unexpected(c, "a valid string", pos));
    }

    owned.as_mut().map(|owned| owned.push(c as char));
    Ok(()).map(|_| true)
}

/// Deserializes the next character as a '\' then an escape character
fn escape<'de>(
    stream: &mut SliceByteCharStream<'de>,
    start_offset: usize,
    owned: &mut Option<String>,
) -> Result<'de, ()> {
    let owned = create_owned(stream, start_offset, owned);

    stream.next().unwrap();

    let pos = stream.position();
    match stream
        .next()
        .map_err(|error| DeserializeError::new(error, pos))?
    {
        Some('"') => Ok(owned.push('"')),
        Some('\\') => Ok(owned.push('\\')),
        Some('/') => Ok(owned.push('/')),
        Some('b') => Ok(owned.push('\x08')),
        Some('f') => Ok(owned.push('\x0C')),
        Some('n') => Ok(owned.push('\n')),
        Some('r') => Ok(owned.push('\r')),
        Some('t') => Ok(owned.push('\t')),
        Some('u') => unicode_escape(stream, owned),
        Some(c) => Err(DeserializeError::unexpected(c, "a valid escape", pos)),
        None => Err(DeserializeError::new(
            DeserializeErrorKind::UnexpectedEndOfJSON,
            pos,
        )),
    }
}

/// Deserializes a unicode escape sequence from `stream` and places it in `owned`
fn unicode_escape<'de>(
    stream: &mut SliceByteCharStream<'de>,
    owned: &mut String,
) -> Result<'de, ()> {
    let pos = stream.position();
    let offset = stream.offset();
    let value = get_four_hex(stream)?;

    if value >= 0xD800 && value <= 0xDBFF {
        surrogate_pair_escape(value as u32, stream, owned)
    } else if value >= 0xDC00 && value <= 0xDFFF {
        Err(DeserializeError::unexpected(
            &stream.bytes()[offset..stream.offset()],
            "a valid utf8 value",
            pos,
        ))
    } else {
        owned.push(unsafe { char::from_u32_unchecked(value as u32) });
        Ok(())
    }
}

/// Deserializes a high surrogate pair from the stream
fn surrogate_pair_escape<'de>(
    high_surrogate: u32,
    stream: &mut SliceByteCharStream<'de>,
    owned: &mut String,
) -> Result<'de, ()> {
    expect(stream, b'\\', "low UTF-16 surrogate")?;
    expect(stream, b'u', "low UTF-16 surrogate")?;

    let pos = stream.position();
    let offset = stream.offset();
    let low_surrogate = get_four_hex(stream)?;

    if low_surrogate < 0xDC00 || low_surrogate > 0xDFFF {
        return Err(DeserializeError::unexpected(
            &stream.bytes()[offset..stream.offset()],
            "",
            pos,
        ));
    }

    let c = 0x10000 + ((high_surrogate - 0xD800) * 0x400) + (low_surrogate as u32 - 0xDC00);
    owned.push(unsafe { char::from_u32_unchecked(c) });
    Ok(())
}

/// Deserializes 4 hex digits from `stream` into a [`u16`]
fn get_four_hex<'de>(stream: &mut SliceByteCharStream<'de>) -> Result<'de, u16> {
    let mut value = 0;

    for _ in 0..4 {
        let pos = stream.position();
        let c = stream
            .next()
            .map_err(|error| DeserializeError::new(error, pos))?
            .ok_or_else(|| DeserializeError::new(DeserializeErrorKind::UnexpectedEndOfJSON, pos))?;

        value <<= 4;

        if c.is_ascii_digit() {
            value += (c as u8 - b'0') as u16;
        } else if c >= 'a' && c <= 'f' {
            value += (c as u8 - b'a' + 10) as u16;
        } else if c >= 'A' && c <= 'F' {
            value += (c as u8 - b'A' + 10) as u16;
        } else {
            return Err(DeserializeError::unexpected(c, "4 hex digits", pos));
        }
    }

    Ok(value)
}
