pub type Result<T> = std::result::Result<T, Error>;

pub enum Error {
    IO(std::io::Error),
    Custom(String),
}

impl Error {
    pub fn io(error: std::io::Error) -> Self {
        Error::IO(error)
    }
}

impl data_format::Error for Error {
    fn custom<T: std::fmt::Display>(error: T) -> Self {
        Error::Custom(error.to_string())
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::IO(error) => Some(error),

            Error::Custom(_) => None,
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::IO(error) => error.fmt(f),
            Error::Custom(error) => error.fmt(f),
        }
    }
}

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self, f)
    }
}
