pub enum DeserializeError<IOError, OtherError> {
    Parse(ParseError<IOError>),
    Other(OtherError),
}

pub enum ParseError<IOError> {
    IO(IOError),
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
        }
    }
}

impl<IOError: std::fmt::Debug> std::fmt::Debug for ParseError<IOError> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::IO(error) => error.fmt(f),
        }
    }
}

impl<IOError> From<IOError> for ParseError<IOError> {
    fn from(error: IOError) -> Self {
        ParseError::IO(error)
    }
}
