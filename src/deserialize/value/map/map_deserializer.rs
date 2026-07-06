use crate::{
    deserialize::{from_value, value::ValueMapDeserializer, ValueDeserializer},
    DeserializeError, Value,
};
use data_format::{MapDeserializer, StringDeserializer};

impl<'de> MapDeserializer<'de> for ValueMapDeserializer<'de> {
    type Error = DeserializeError<'de>;

    type Value = Value<'de>;

    type ValueDeserializer = ValueDeserializer<'de>;

    fn next_key<K: data_format::Deserialize<'de>>(&mut self) -> Result<Option<K>, Self::Error> {
        assert!(self.next_value.is_none());

        if let Some((key, value)) = self.map.next() {
            self.next_value = Some(value);

            return K::deserialize(StringDeserializer::new(key)).map(Some);
        }

        Ok(None)
    }

    fn next_value<V: data_format::Deserialize<'de>>(&mut self) -> Result<V, Self::Error> {
        assert!(self.next_value.is_some());
        from_value(self.next_value.take().unwrap())
    }

    fn size_hint(&self) -> Option<usize> {
        Some(self.map.len())
    }
}
