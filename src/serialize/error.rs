/// The result of deserializing from JSON
pub type Result<T> = std::result::Result<T, SerializeError>;

/// An error that can occur while serializing
pub enum SerializeError {
    /// An error ocurred while reading
    IO(std::io::Error),

    /// A custom error
    Custom(String),
}

impl SerializeError {
    /// Creates a new [`SerializationError::IO`]
    pub fn io(error: std::io::Error) -> Self {
        SerializeError::IO(error)
    }
}

impl data_format::SerializeError for SerializeError {
    fn custom<T: std::fmt::Display>(error: T) -> Self {
        SerializeError::Custom(error.to_string())
    }
}

impl std::error::Error for SerializeError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            SerializeError::IO(error) => Some(error),

            SerializeError::Custom(_) => None,
        }
    }
}

impl std::fmt::Display for SerializeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SerializeError::IO(error) => error.fmt(f),
            SerializeError::Custom(error) => f.write_str(error),
        }
    }
}

impl std::fmt::Debug for SerializeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self, f)
    }
}