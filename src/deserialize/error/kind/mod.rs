use data_format::Unexpected;
use lct_streams::InvalidUtf8;
use std::borrow::Cow;

mod display;
mod from;

/// A kind of error that can occur while deserializing
#[derive(Debug)]
pub enum DeserializeErrorKind<'de> {
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
        unexpected: Unexpected,

        /// The expected value
        expected: &'static str,
    },

    /// The end of the JSON stream was reach unexpectedly
    UnexpectedEndOfJSON,

    /// The stream contains invalid UTF8
    InvalidUtf8(InvalidUtf8),

    /// A custom error
    Custom(String),
}
