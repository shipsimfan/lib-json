use super::{Result, Stream};
use crate::DeserializeError;

/// A deserialized number
struct Number {
    /// Did the number start with a minus? AKA Is it negative?
    pub(self) minus: bool,

    /// The integer portion of the number
    pub(self) int: usize,

    /// The fractional portion of the number
    pub(self) frac: Option<usize>,

    /// The exponential portion of the number
    pub(self) exp: Option<isize>,
}

/// Deserializes an [`f64`] from `stream`
pub(super) fn deserialize_f64<'de>(stream: &mut Stream<'de>) -> Result<'de, f64> {
    stream.skip_whitespace();
    let start_index = stream.index();

    Number::deserialize(stream)?;

    // Let rust parse the float
    Ok(
        unsafe { std::str::from_utf8_unchecked(stream.get_bytes(start_index)) }
            .parse()
            .unwrap(),
    )
}

/// Deserializes an [`isize`] from `stream`
pub(super) fn deserialize_isize<'de>(stream: &mut Stream<'de>) -> Result<'de, isize> {
    stream.skip_whitespace();
    let number = Number::deserialize(stream)?;

    if number.frac.is_some() || number.exp.is_some() {
        return Err(DeserializeError::InvalidType {
            unexpected: "fractional number".into(),
            expected: "an integer".into(),
        });
    }

    Ok(number.int as isize * if number.minus { -1 } else { 1 })
}

/// Deserializes an [`usize`] from `stream`
pub(super) fn deserialize_usize<'de>(stream: &mut Stream<'de>) -> Result<'de, usize> {
    stream.skip_whitespace();
    let number = Number::deserialize(stream)?;

    if number.frac.is_some() || number.exp.is_some() {
        return Err(DeserializeError::InvalidType {
            unexpected: "fractional number".into(),
            expected: "a positive integer".into(),
        });
    }

    if number.minus {
        return Err(DeserializeError::InvalidType {
            unexpected: "negative integer".into(),
            expected: "a positive integer".into(),
        });
    }

    Ok(number.int)
}

impl Number {
    /// Deserializes a [`Number`] from `stream`
    pub(self) fn deserialize<'de>(stream: &mut Stream<'de>) -> Result<'de, Self> {
        let (first_digit, minus) = Number::deserialize_first_digit(stream)?;

        let int = if first_digit == 0 {
            0
        } else {
            Number::deserialize_int(stream, Some(first_digit))?
        };

        let is_frac = match stream.peek() {
            Some(b'.') => {
                stream.next();
                true
            }
            Some(b'e') | Some(b'E') => {
                stream.next();
                false
            }
            _ => {
                return Ok(Number {
                    minus,
                    int,
                    frac: None,
                    exp: None,
                });
            }
        };

        let frac = if is_frac {
            Some(Number::deserialize_int(stream, None)?)
        } else {
            None
        };

        if is_frac {
            match stream.peek() {
                Some(b'e') | Some(b'E') => {
                    stream.next();
                }
                _ => {
                    return Ok(Number {
                        minus,
                        int,
                        frac,
                        exp: None,
                    });
                }
            }
        }

        let exp_minus = match stream.peek() {
            Some(b'-') => {
                stream.next();
                true
            }
            Some(b'+') => {
                stream.next();
                false
            }
            _ => false,
        };

        let exp = Number::deserialize_int(stream, None)? as isize;

        Ok(Number {
            minus,
            int,
            frac,
            exp: Some(exp * if exp_minus { -1 } else { 1 }),
        })
    }

    /// Gets the first digit of the number, returning a boolean as well indicating if there was a
    /// minus
    fn deserialize_first_digit<'de>(stream: &mut Stream<'de>) -> Result<'de, (u8, bool)> {
        match Number::deserialize_next_number(stream, Some(b'-'))? {
            Some(digit) => return Ok((digit, false)),
            None => {}
        }

        Ok((
            Number::deserialize_next_number(stream, None)?.unwrap(),
            true,
        ))
    }

    /// Deserializes an integer from the stream
    fn deserialize_int<'de>(
        stream: &mut Stream<'de>,
        first_digit: Option<u8>,
    ) -> Result<'de, usize> {
        let (mut value, mut count) = match first_digit {
            Some(first_digit) => (first_digit as usize, 1),
            None => (0, 0),
        };

        while let Some(c) = stream.peek() {
            if !c.is_ascii_digit() {
                break;
            }

            stream.next();

            value *= 10;
            value += (c - b'0') as usize;
            count += 1;
        }

        if count == 0 {
            return Err(match stream.peek() {
                Some(_) => DeserializeError::Unexpected {
                    unexpected: stream.get_next_byte(),
                    expected: "a",
                },
                None => DeserializeError::UnexpectedEndOfJSON,
            });
        }

        Ok(value)
    }

    /// Deserializes the next number from the stream, or `other` if passed.
    ///
    /// The returned value is the number the digit represents, not its character value.
    ///
    /// If `other` is provided, a return of [`None`] indicates that the next character was `other`.
    /// If `other` is not provided, an [`Ok`] return will always be [`Some`] and is safe to unwrap.
    fn deserialize_next_number<'de>(
        stream: &mut Stream<'de>,
        other: Option<u8>,
    ) -> Result<'de, Option<u8>> {
        let c = stream.peek().ok_or(DeserializeError::UnexpectedEndOfJSON)?;

        if let Some(other) = other {
            if c == other {
                stream.next();
                return Ok(None);
            }
        }

        if !c.is_ascii_digit() {
            return Err(DeserializeError::Unexpected {
                unexpected: stream.get_next_byte(),
                expected: "a number",
            });
        }

        stream.next();
        Ok(Some(c - b'0'))
    }
}
