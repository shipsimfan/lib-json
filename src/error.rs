use data_format::{Expected, Unexpected};

pub type Result<T> = std::result::Result<T, Error>;

pub enum Error {
    InvalidType {
        unexpected: String,
        expected: String,
    },
    InvalidValue {
        unexpected: String,
        expected: String,
    },
    InvalidLength {
        unexpected: usize,
        expected: String,
    },
    UnknownField {
        field: String,
        expected: &'static [&'static str],
    },
    MissingField(&'static str),
    DuplicateField(&'static str),
    IO(std::io::Error),
    Custom(String),
}

struct ExpectedDisplay<'a>(&'a dyn Expected);

impl Error {
    pub fn io(error: std::io::Error) -> Self {
        Error::IO(error)
    }
}

impl data_format::Error for Error {
    fn custom<T: std::fmt::Display>(error: T) -> Self {
        Error::Custom(error.to_string())
    }

    fn invalid_type(unexpected: Unexpected<'_>, expected: &dyn Expected) -> Self {
        Error::InvalidType {
            unexpected: unexpected.to_string(),
            expected: ExpectedDisplay(expected).to_string(),
        }
    }

    fn invalid_value(unexpected: Unexpected<'_>, expected: &dyn Expected) -> Self {
        Error::InvalidValue {
            unexpected: unexpected.to_string(),
            expected: ExpectedDisplay(expected).to_string(),
        }
    }

    fn invalid_length(unexpected: usize, expected: &dyn Expected) -> Self {
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
            | Error::DuplicateField(_) => None,
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
        }
    }
}

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self, f)
    }
}

impl<'a> std::fmt::Display for ExpectedDisplay<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
