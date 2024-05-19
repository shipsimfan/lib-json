use super::Stream;
use crate::{Error, Result};

pub(super) fn deserialize_string<'a>(stream: &mut Stream<'a>) -> Result<String> {
    stream.expect(b'"', stream.index(), "a string")?;

    let start_index = stream.index();
    let mut string = Vec::new();
    loop {
        let c = match stream.next() {
            Some(b'"') => break,
            Some(c) => c,
            None => return Err(Error::UnexpectedEndOfJSON),
        };

        if c < 0x20 {
            return Err(Error::UnexpectedCharacter {
                unexpected: stream.get_bytes(start_index).to_owned(),
                expected: "a valid string",
            });
        }

        if c != b'\\' {
            string.push(c);
            continue;
        }

        match stream.next() {
            Some(b'"') => string.push(b'"'),
            Some(b'\\') => string.push(b'\\'),
            Some(b'/') => string.push(b'/'),
            Some(b'b') => string.push(b'\x08'),
            Some(b'f') => string.push(b'\x0C'),
            Some(b'n') => string.push(b'\x0A'),
            Some(b'r') => string.push(b'\x0D'),
            Some(b't') => string.push(b'\x09'),
            Some(b'u') => {
                let mut val = 0;
                for _ in 0..4 {
                    let c = match stream.next().ok_or(Error::UnexpectedEndOfJSON)? {
                        c if c.is_ascii_digit() => c - b'0',
                        _ => {
                            return Err(Error::UnexpectedCharacter {
                                unexpected: stream.get_bytes(start_index).to_owned(),
                                expected: "a digit",
                            })
                        }
                    };

                    val <<= 8;
                    val |= c as u32;
                }

                let mut buffer = [0; 4];
                string.extend_from_slice(
                    char::from_u32(val)
                        .unwrap_or(char::REPLACEMENT_CHARACTER)
                        .encode_utf8(&mut buffer)
                        .as_bytes(),
                );
            }
            Some(_) => {
                return Err(Error::UnexpectedCharacter {
                    unexpected: stream.get_bytes(start_index).to_owned(),
                    expected: "a valid escape",
                })
            }
            None => return Err(Error::UnexpectedEndOfJSON),
        }
    }

    Ok(String::from_utf8_lossy(&string).to_string())
}
