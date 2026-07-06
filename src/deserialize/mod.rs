use crate::Value;
use data_format::Deserialize;
use deserializer::Deserializer;
use error::Result;
use lct_streams::SliceByteCharStream;
use list::ListDeserializer;
use map::MapDeserializer;
use value::ValueDeserializer;

use utility::*;

mod deserializer;
mod error;
mod list;
mod map;
mod number;
mod string;
mod value;

mod utility;

pub use error::{DeserializeError, DeserializeErrorKind};

/// Attempts to deserialize `string` as JSON into `T`
pub fn from_str<'de, T: Deserialize<'de>>(string: &'de str) -> Result<'de, T> {
    from_bytes(string.as_bytes())
}

/// Attempts to deserialize `bytes` as JSON into `T`
pub fn from_bytes<'de, T: Deserialize<'de>>(bytes: &'de [u8]) -> Result<'de, T> {
    let mut stream = SliceByteCharStream::new(bytes);

    T::deserialize(Deserializer::new(&mut stream)).map_err(|error| {
        debug_assert!(error.position().is_some());
        error
    })
}

pub fn from_value<'de, T: Deserialize<'de>>(value: Value<'de>) -> Result<'de, T> {
    T::deserialize::<ValueDeserializer<'de>>(data_format::ValueDeserializer::new(value))
}
