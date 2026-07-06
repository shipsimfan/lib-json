use crate::Value;
use list::ValueListDeserializer;
use map::ValueMapDeserializer;

mod list;
mod map;

mod deserializer;
mod value_deserializer;

/// A deserializer for a JSON [`Value`]
pub(in crate::deserialize) struct ValueDeserializer<'de> {
    /// The value to be deserialized
    value: Value<'de>,
}
