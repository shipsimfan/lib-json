#[cfg(feature = "no_std")]
use alloc::string::{String, ToString};
#[cfg(feature = "no_std")]
use core::fmt::Error;
#[cfg(not(feature = "no_std"))]
use std::io::Error;

/// The result of deserializing from JSON
pub type Result<T> = core::result::Result<T, SerializeError>;

/// An error that can occur while serializing
pub enum SerializeError {
    /// An error ocurred while reading
    IO(Error),

    /// A custom error
    Custom(String),
}

impl SerializeError {
    /// Creates a new [`SerializationError::IO`]
    pub fn io(error: Error) -> Self {
        SerializeError::IO(error)
    }
}

impl data_format::SerializeError for SerializeError {
    fn custom<T: core::fmt::Display>(error: T) -> Self {
        SerializeError::Custom(error.to_string())
    }
}

impl core::error::Error for SerializeError {
    fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
        match self {
            SerializeError::IO(error) => Some(error),

            SerializeError::Custom(_) => None,
        }
    }
}

impl core::fmt::Display for SerializeError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            SerializeError::IO(error) => error.fmt(f),
            SerializeError::Custom(error) => f.write_str(error),
        }
    }
}

impl core::fmt::Debug for SerializeError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Display::fmt(self, f)
    }
}
