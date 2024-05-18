use data_format::{Expected, Unexpected};

/// The result of serializing into JSON or deserializing from JSON
pub type Result<T> = std::result::Result<T, Error>;

/// An error that can occur while serializing or deserializing
pub enum Error {
    /// The provided type doesn't match the expected type(s)
    InvalidType {
        /// The unexpected provided type
        unexpected: String,

        /// The expected type
        expected: String,
    },

    /// The provided value is invalid
    InvalidValue {
        /// The unexpected provided value
        unexpected: String,

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
        field: String,

        /// The valid field names
        expected: &'static [&'static str],
    },

    /// A required field in a map is missing
    MissingField(&'static str),

    /// A field in a map has been duplicated
    DuplicateField(&'static str),

    /// An unexpected character was encountered
    UnexpectedCharacter {
        /// The error causing string
        unexpected: Vec<u8>,

        /// The expected value
        expected: &'static str,
    },

    /// The end of the JSON stream was reach unexpectedly
    UnexpectedEndOfJSON,

    /// An error ocurred while reading or writing
    IO(std::io::Error),

    /// A custom error
    Custom(String),
}

/// Converts the [`Expected`] trait into the [`std::fmt::Display`] trait so it can be converted to
/// a string.
struct ExpectedDisplay<'a, E: Expected + ?Sized>(&'a E);

impl Error {
    /// Creates a new [`Error::IO`]
    pub fn io(error: std::io::Error) -> Self {
        Error::IO(error)
    }
}

impl data_format::Error for Error {
    fn custom<T: std::fmt::Display>(error: T) -> Self {
        Error::Custom(error.to_string())
    }

    fn invalid_type<'a, U: Into<Unexpected<'a>>, E: Expected + ?Sized>(
        unexpected: U,
        expected: &E,
    ) -> Self {
        Error::InvalidType {
            unexpected: unexpected.into().to_string(),
            expected: ExpectedDisplay(expected).to_string(),
        }
    }

    fn invalid_value<'a, U: Into<Unexpected<'a>>, E: Expected + ?Sized>(
        unexpected: U,
        expected: &E,
    ) -> Self {
        Error::InvalidValue {
            unexpected: unexpected.into().to_string(),
            expected: ExpectedDisplay(expected).to_string(),
        }
    }

    fn invalid_length<E: Expected + ?Sized>(unexpected: usize, expected: &E) -> Self {
        Error::InvalidLength {
            unexpected,
            expected: ExpectedDisplay(expected).to_string(),
        }
    }

    fn unknown_field(field: &str, expected: &'static [&'static str]) -> Self {
        Error::UnknownField {
            field: field.to_string(),
            expected,
        }
    }

    fn missing_field(field: &'static str) -> Self {
        Error::MissingField(field)
    }

    fn duplicate_field(field: &'static str) -> Self {
        Error::DuplicateField(field)
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::IO(error) => Some(error),

            Error::Custom(_)
            | Error::InvalidType {
                unexpected: _,
                expected: _,
            }
            | Error::InvalidValue {
                unexpected: _,
                expected: _,
            }
            | Error::InvalidLength {
                unexpected: _,
                expected: _,
            }
            | Error::UnknownField {
                field: _,
                expected: _,
            }
            | Error::MissingField(_)
            | Error::DuplicateField(_)
            | Error::UnexpectedCharacter {
                unexpected: _,
                expected: _,
            }
            | Error::UnexpectedEndOfJSON => None,
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::IO(error) => error.fmt(f),
            Error::Custom(error) => error.fmt(f),

            Error::InvalidType {
                unexpected,
                expected,
            } => write!(
                f,
                "unexpected type \"{}\", expected {}",
                unexpected, expected
            ),
            Error::InvalidValue {
                unexpected,
                expected,
            } => write!(
                f,
                "unexpected value \"{}\", expected {}",
                unexpected, expected
            ),
            Error::InvalidLength {
                unexpected,
                expected,
            } => write!(f, "unexpected length {}, expected {}", unexpected, expected),
            Error::UnknownField { field, expected } => {
                write!(f, "unknown field \"{}\", expected ", field)?;
                for i in 0..expected.len() {
                    if i != 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", expected[i])?;
                }
                Ok(())
            }
            Error::MissingField(field) => write!(f, "missing field \"{}\"", field),
            Error::DuplicateField(field) => write!(f, "\"{}\" appears more than once", field),
            Error::UnexpectedCharacter {
                unexpected,
                expected,
            } => write!(
                f,
                "unexpected \"{}\", expected \"{}\"",
                String::from_utf8_lossy(unexpected),
                expected
            ),
            Error::UnexpectedEndOfJSON => write!(f, "unexpected end of JSON"),
        }
    }
}

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self, f)
    }
}

impl<'a, E: Expected + ?Sized> std::fmt::Display for ExpectedDisplay<'a, E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
