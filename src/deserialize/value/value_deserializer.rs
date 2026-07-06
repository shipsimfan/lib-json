use crate::{deserialize::ValueDeserializer, Value};

impl<'de> data_format::ValueDeserializer<'de> for ValueDeserializer<'de> {
    type Value = Value<'de>;

    fn new(value: Self::Value) -> Self {
        ValueDeserializer { value }
    }
}
