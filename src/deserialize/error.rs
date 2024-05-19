use data_format::{Expected, Unexpected};
use std::borrow::Cow;

/// The result of deserializing from JSON
pub type Result<'de, T> = std::result::Result<T, DeserializeError<'de>>;

/// An error that can occur while deserializing
pub enum DeserializeError<'de> {
    /// The provided type doesn't match the expected type(s)
    InvalidType {
        /// The unexpected provided type
        unexpected: Unexpected,

        /// The expected type
        expected: String,
    },

    /// The provided value is invalid
    InvalidValue {
        /// The unexpected provided value
        unexpected: Unexpected,

        /// The value(s) that were expected
        expected: String,
    },

    /// The length of an array is invalid
    InvalidLength {
        /// The unexpected length
        unexpected: usize,

        /// The expected length
        expected: String,
    },

    /// A field in a map is unknown
    UnknownField {
        /// The name of the unknown field
        field: Cow<'de, str>,

        /// The valid field names
        expected: &'static [&'static str],
    },

    /// A required field in a map is missing
    MissingField(&'static str),

    /// A field in a map has been duplicated
    DuplicateField(&'static str),

    /// An unexpected character was encountered
    Unexpected {
        /// The error causing string
        unexpected: &'de [u8],

        /// The expected value
        expected: &'static str,
    },

    /// The end of the JSON stream was reach unexpectedly
    UnexpectedEndOfJSON,

    /// A custom error
    Custom(String),
}

impl<'de> data_format::DeserializeError<'de> for DeserializeError<'de> {
    fn custom<T: std::fmt::Display>(error: T) -> Self {
        DeserializeError::Custom(error.to_string())
    }

    fn invalid_type<U: Into<Unexpected>, E: Expected + ?Sized>(
        unexpected: U,
        expected: &E,
    ) -> Self {
        DeserializeError::InvalidType {
            unexpected: unexpected.into(),
            expected: expected.to_string(),
        }
    }

    fn invalid_value<'a, U: Into<Unexpected>, E: Expected + ?Sized>(
        unexpected: U,
        expected: &E,
    ) -> Self {
        DeserializeError::InvalidValue {
            unexpected: unexpected.into(),
            expected: expected.to_string(),
        }
    }

    fn invalid_length<E: Expected + ?Sized>(unexpected: usize, expected: &E) -> Self {
        DeserializeError::InvalidLength {
            unexpected,
            expected: expected.to_string(),
        }
    }

    fn unknown_field<S: Into<Cow<'de, str>>>(field: S, expected: &'static [&'static str]) -> Self {
        DeserializeError::UnknownField {
            field: field.into(),
            expected,
        }
    }

    fn missing_field(field: &'static str) -> Self {
        DeserializeError::MissingField(field)
    }

    fn duplicate_field(field: &'static str) -> Self {
        DeserializeError::DuplicateField(field)
    }
}

impl<'de> std::error::Error for DeserializeError<'de> {}

impl<'de> std::fmt::Display for DeserializeError<'de> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DeserializeError::Custom(error) => f.write_str(error),

            DeserializeError::InvalidType {
                unexpected,
                expected,
            } => write!(
                f,
                "unexpected type \"{}\", expected {}",
                unexpected, expected
            ),
            DeserializeError::InvalidValue {
                unexpected,
                expected,
            } => write!(
                f,
                "unexpected value \"{}\", expected {}",
                unexpected, expected
            ),
            DeserializeError::InvalidLength {
                unexpected,
                expected,
            } => write!(f, "unexpected length {}, expected {}", unexpected, expected),
            DeserializeError::UnknownField { field, expected } => {
                write!(f, "unknown field \"{}\", expected ", field)?;
                for i in 0..expected.len() {
                    if i != 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", expected[i])?;
                }
                Ok(())
            }
            DeserializeError::MissingField(field) => write!(f, "missing field \"{}\"", field),
            DeserializeError::DuplicateField(field) => {
                write!(f, "\"{}\" appears more than once", field)
            }
            DeserializeError::Unexpected {
                unexpected,
                expected,
            } => write!(
                f,
                "unexpected \"{}\", expected \"{}\"",
                String::from_utf8_lossy(unexpected),
                expected
            ),
            DeserializeError::UnexpectedEndOfJSON => write!(f, "unexpected end of JSON"),
        }
    }
}

impl<'de> std::fmt::Debug for DeserializeError<'de> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self, f)
    }
}
