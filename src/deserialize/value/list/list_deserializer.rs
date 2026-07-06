use crate::{
    deserialize::{from_value, value::ValueListDeserializer},
    DeserializeError,
};
use data_format::ListDeserializer;

impl<'de> ListDeserializer<'de> for ValueListDeserializer<'de> {
    type Error = DeserializeError<'de>;

    fn next_item<T: data_format::Deserialize<'de>>(&mut self) -> Result<Option<T>, Self::Error> {
        if self.list.len() == 0 {
            return Ok(None);
        }

        from_value(self.list.swap_remove(0)).map(Some)
    }

    fn size_hint(&self) -> Option<usize> {
        Some(self.list.len())
    }
}
