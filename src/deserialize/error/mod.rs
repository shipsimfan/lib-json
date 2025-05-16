use lct_streams::Position;

mod deserialize_error;
mod display;
mod from;
mod get;
mod kind;
mod new;
mod set;

pub use kind::DeserializeErrorKind;

/// The result of deserializing from JSON
pub type Result<'de, T> = std::result::Result<T, DeserializeError<'de>>;

/// An error that occurred while deserializing
#[derive(Debug)]
pub struct DeserializeError<'de> {
    /// The kind of error that occurred
    kind: DeserializeErrorKind<'de>,

    /// Where the error occurred
    position: Option<Position>,
}

impl<'de> std::error::Error for DeserializeError<'de> {}
