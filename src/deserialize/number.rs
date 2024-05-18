use super::Stream;
use crate::{Error, Result};

/// A deserialized number
struct Number {
    /// Did the number start with a minus? AKA Is it negative?
    pub(self) minus: bool,

    /// The integer portion of the number
    pub(self) int: usize,

    /// The fractional portion of the number
    pub(self) frac: Option<usize>,

    /// The exponential portion of the number
    pub(self) exp: Option<usize>,
}

/// Deserializes an [`f64`] from `stream`
pub(super) fn deserialize_f64(stream: &mut Stream) -> Result<f64> {
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
pub(super) fn deserialize_isize(stream: &mut Stream) -> Result<isize> {
    let number = Number::deserialize(stream)?;

    if number.frac.is_some() || number.exp.is_some() {
        return Err(Error::InvalidType {
            unexpected: "fractional number".into(),
            expected: "an integer".into(),
        });
    }

    Ok(number.int as isize * if number.minus { -1 } else { 1 })
}

/// Deserializes an [`usize`] from `stream`
pub(super) fn deserialize_usize(stream: &mut Stream) -> Result<usize> {
    let number = Number::deserialize(stream)?;

    if number.frac.is_some() || number.exp.is_some() {
        return Err(Error::InvalidType {
            unexpected: "fractional number".into(),
            expected: "a positive integer".into(),
        });
    }

    if number.minus {
        return Err(Error::InvalidType {
            unexpected: "negative integer".into(),
            expected: "a positive integer".into(),
        });
    }

    Ok(number.int)
}

impl Number {
    /// Deserializes a [`Number`] from `stream`
    pub(self) fn deserialize(stream: &mut Stream) -> Result<Self> {
        todo!()
    }
}
