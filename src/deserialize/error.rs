use crate::Position;

pub enum DeserializeError<IOError, OtherError> {
    Parse(ParseError<IOError>),
    Other(OtherError),
}

pub enum ParseError<IOError> {
    IO(IOError),

    UnexpectedCharacter(char, Position),
    UnexpectedEndOfStream(Position),
    InvalidUTF8(Position),
    InvalidNumber(Position),
}

impl<IOError: std::error::Error, OtherError: std::error::Error> std::error::Error
    for DeserializeError<IOError, OtherError>
{
}

impl<IOError: std::fmt::Display, OtherError: std::fmt::Display> std::fmt::Display
    for DeserializeError<IOError, OtherError>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DeserializeError::Parse(error) => error.fmt(f),
            DeserializeError::Other(error) => error.fmt(f),
        }
    }
}

impl<IOError: std::fmt::Debug, OtherError: std::fmt::Debug> std::fmt::Debug
    for DeserializeError<IOError, OtherError>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DeserializeError::Parse(error) => error.fmt(f),
            DeserializeError::Other(error) => error.fmt(f),
        }
    }
}

impl<IOError: std::error::Error> std::error::Error for ParseError<IOError> {}

impl<IOError: std::fmt::Display> std::fmt::Display for ParseError<IOError> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::IO(error) => error.fmt(f),

            ParseError::UnexpectedCharacter(c, position) => {
                write!(f, "unexpected character ")?;
                if c.is_ascii_graphic() || *c == ' ' {
                    write!(f, "\'{}\' ({:#X})", c, *c as u32)
                } else {
                    write!(f, "{:#X}", *c as u32)
                }?;
                write!(f, " at {}", position)
            }
            ParseError::UnexpectedEndOfStream(position) => {
                write!(f, "unexpected end of stream at {}", position)
            }
            ParseError::InvalidUTF8(position) => write!(f, "invalid UTF-8 at {}", position),
            ParseError::InvalidNumber(position) => write!(f, "invalid number at {}", position),
        }
    }
}

impl<IOError: std::fmt::Debug> std::fmt::Debug for ParseError<IOError> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::IO(error) => error.fmt(f),

            ParseError::UnexpectedCharacter(_, _)
            | ParseError::UnexpectedEndOfStream(_)
            | ParseError::InvalidUTF8(_)
            | ParseError::InvalidNumber(_) => self.fmt(f),
        }
    }
}

impl<IOError> From<IOError> for ParseError<IOError> {
    fn from(error: IOError) -> Self {
        ParseError::IO(error)
    }
}
