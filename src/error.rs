use crate::{Position, Token, Type};

pub enum Error {
    // Parse Errors
    UnexpectedEndOfStream(Position),
    UnexpectedCharacter(u8, Position),
    UnexpectedToken(Token, Position),
    InvalidUTF8(std::string::FromUtf8Error, Position),

    // Deserialize Errors
    InvalidType(Option<String>, Type, Type),
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::InvalidUTF8(error, _) => Some(error),
            Error::UnexpectedCharacter(_, _)
            | Error::UnexpectedEndOfStream(_)
            | Error::UnexpectedToken(_, _) => None,
            Error::InvalidType(_, _, _) => None,
        }
    }
}

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self, f)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::UnexpectedEndOfStream(position) => {
                write!(f, "Unexpected end of stream at {}", position)
            }
            Error::UnexpectedCharacter(char, position) => {
                write!(
                    f,
                    "Unexpected character '{}' at {}",
                    *char as char, position
                )
            }
            Error::UnexpectedToken(class, position) => {
                write!(f, "Unexpected {} at {}", class, position)
            }
            Error::InvalidUTF8(error, position) => {
                write!(f, "Invalid UTF-8 in string at {} ({})", error, position)
            }
            Error::InvalidType(key, expected, actual) => {
                write!(
                    f,
                    "Invalid type{}. Expected {}, instead found {}",
                    match key {
                        Some(key) => format!(" at \"{}\"", key),
                        None => String::new(),
                    },
                    expected,
                    actual
                )
            }
        }
    }
}
