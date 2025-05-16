use crate::{
    deserialize::{peek, skip_whitespace, Result},
    DeserializeError, DeserializeErrorKind,
};
use lct_streams::{Position, SliceByteCharStream};

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
pub(super) fn deserialize_f64<'de>(
    stream: &mut SliceByteCharStream<'de>,
) -> Result<'de, (f64, Position)> {
    skip_whitespace(stream)?;
    let pos = stream.position();
    let start_offset = stream.offset();

    Number::deserialize(stream)?;

    // Let rust parse the float
    Ok((
        unsafe { std::str::from_utf8_unchecked(&stream.bytes()[start_offset..stream.offset()]) }
            .parse()
            .unwrap(),
        pos,
    ))
}

/// Deserializes an [`isize`] from `stream`
pub(super) fn deserialize_isize<'de>(
    stream: &mut SliceByteCharStream<'de>,
) -> Result<'de, (isize, Position)> {
    skip_whitespace(stream)?;
    let pos = stream.position();
    let start_offset = stream.offset();
    let number = Number::deserialize(stream)?;

    if number.frac.is_some() || number.exp.is_some() {
        return Err(DeserializeError::new(
            DeserializeErrorKind::InvalidType {
                unexpected: stream.bytes()[start_offset..stream.offset()].into(),
                expected: "an integer".into(),
            },
            pos,
        ));
    }

    Ok((number.int as isize * if number.minus { -1 } else { 1 }, pos))
}

/// Deserializes an [`usize`] from `stream`
pub(super) fn deserialize_usize<'de>(
    stream: &mut SliceByteCharStream<'de>,
) -> Result<'de, (usize, Position)> {
    skip_whitespace(stream)?;
    let pos = stream.position();
    let start_offset = stream.offset();
    let number = Number::deserialize(stream)?;

    if number.frac.is_some() || number.exp.is_some() || number.minus {
        return Err(DeserializeError::new(
            DeserializeErrorKind::InvalidType {
                unexpected: stream.bytes()[start_offset..stream.offset()].into(),
                expected: "a positive integer".into(),
            },
            pos,
        ));
    }

    Ok((number.int, pos))
}

impl Number {
    /// Deserializes a [`Number`] from `stream`
    pub(self) fn deserialize<'de>(stream: &mut SliceByteCharStream<'de>) -> Result<'de, Self> {
        let (first_digit, minus) = Number::deserialize_first_digit(stream)?;

        let int = if first_digit == 0 {
            0
        } else {
            Number::deserialize_int(stream, Some(first_digit))?
        };

        let is_frac = match stream
            .peek()
            .map_err(|error| DeserializeError::new(error, stream.position()))?
        {
            Some('.') => {
                stream.next().unwrap();
                true
            }
            Some('e') | Some('E') => {
                stream.next().unwrap();
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
            match stream
                .peek()
                .map_err(|error| DeserializeError::new(error, stream.position()))?
            {
                Some('e') | Some('E') => {
                    stream.next().unwrap();
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

        let exp_minus = match stream
            .peek()
            .map_err(|error| DeserializeError::new(error, stream.position()))?
        {
            Some('-') => {
                stream.next().unwrap();
                true
            }
            Some('+') => {
                stream.next().unwrap();
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
    fn deserialize_first_digit<'de>(
        stream: &mut SliceByteCharStream<'de>,
    ) -> Result<'de, (u8, bool)> {
        match Number::deserialize_next_number(stream, Some('-'))? {
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
        stream: &mut SliceByteCharStream<'de>,
        first_digit: Option<u8>,
    ) -> Result<'de, usize> {
        let (mut value, mut count) = match first_digit {
            Some(first_digit) => (first_digit as usize, 1),
            None => (0, 0),
        };

        while let Some(c) = stream
            .peek()
            .map_err(|error| DeserializeError::new(error, stream.position()))?
        {
            if !c.is_ascii_digit() {
                break;
            }

            stream.next().unwrap();

            value *= 10;
            value += (c as u8 - b'0') as usize;
            count += 1;
        }

        if count == 0 {
            return Err(match stream.peek() {
                Ok(Some(c)) => DeserializeError::unexpected(c, "a", stream.position()),
                Ok(None) => DeserializeError::new(
                    DeserializeErrorKind::UnexpectedEndOfJSON,
                    stream.position(),
                ),
                Err(error) => DeserializeError::new(error, stream.position()),
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
        stream: &mut SliceByteCharStream<'de>,
        other: Option<char>,
    ) -> Result<'de, Option<u8>> {
        let (c, pos) = peek(stream)?;

        if let Some(other) = other {
            if c == other {
                stream.next().unwrap();
                return Ok(None);
            }
        }

        if !c.is_ascii_digit() {
            return Err(DeserializeError::unexpected(c, "a number", pos));
        }

        stream.next().unwrap();
        Ok(Some(c as u8 - b'0'))
    }
}
