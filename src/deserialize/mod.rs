use crate::Result;
use data_format::Deserialize;
use deserializer::Deserializer;
use list::ListDeserializer;
use map::MapDeserializer;
use stream::Stream;

mod deserializer;
mod list;
mod map;
mod number;
mod stream;
mod string;

/// Attempts to deserialize `string` as JSON into `T`
pub fn from_str<'de, T: Deserialize<'de>>(string: &'de str) -> Result<T> {
    from_bytes(string.as_bytes())
}

/// Attempts to deserialize `bytes` as JSON into `T`
pub fn from_bytes<'de, T: Deserialize<'de>>(bytes: &'de [u8]) -> Result<T> {
    let mut stream = Stream::new(bytes);

    T::deserialize(Deserializer::new(&mut stream))
}
