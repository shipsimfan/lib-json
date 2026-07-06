use crate::Value;
#[cfg(feature = "no-std")]
use alloc::vec::Vec;

mod list_deserializer;
mod new;

/// A deserializer for a JSON [`Value`] that is a list
pub(in crate::deserialize::value) struct ValueListDeserializer<'de> {
    /// The list to be deserialized
    list: Vec<Value<'de>>,
}
