use crate::Value;

mod error;
mod input;
mod parser;
mod position;

pub use error::{DeserializeError, ParseError};
pub use input::Input;
pub use parser::parse;
pub use position::Position;

pub fn deserialize<I: Input, T: TryFrom<Value<'static>>>(
    input: &mut I,
) -> Result<T, DeserializeError<I::Error, T::Error>> {
    parse(input)
        .map_err(|error| DeserializeError::Parse(error))?
        .try_into()
        .map_err(|error| DeserializeError::Other(error))
}
