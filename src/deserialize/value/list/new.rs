use crate::{deserialize::value::ValueListDeserializer, Value};
#[cfg(feature = "no-std")]
use alloc::vec::Vec;

impl<'de> ValueListDeserializer<'de> {
    /// Creates a new [`ValueListDeserializer`]
    pub fn new(list: Vec<Value<'de>>) -> ValueListDeserializer<'de> {
        ValueListDeserializer { list }
    }
}
