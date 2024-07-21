use super::{Formatter, Result, Serializer};
use crate::SerializeError;
use data_format::Serialize;
use std::io::Write;

/// Serializes maps into JSON using a [`Formatter`]
pub(super) struct MapSerializer<'a, W: Write, F: Formatter> {
    serializer: &'a mut Serializer<W, F>,
}

impl<'a, W: Write, F: Formatter> MapSerializer<'a, W, F> {
    /// Creates a new [`MapSerializer`] with a length hint `len`
    pub(super) fn new(serializer: &'a mut Serializer<W, F>, len: Option<usize>) -> Result<Self> {
        serializer
            .formatter
            .write_object_begin(&mut serializer.output, len)
            .map(|_| MapSerializer { serializer })
            .map_err(SerializeError::io)
    }
}

impl<'a, W: Write, F: Formatter> data_format::MapSerializer for MapSerializer<'a, W, F> {
    type Ok = ();
    type Error = SerializeError;

    fn serialize_entry<K: Serialize + ?Sized, V: Serialize + ?Sized>(
        &mut self,
        key: &K,
        value: &V,
    ) -> Result<()> {
        if !value.map_entry() {
            return Ok(());
        }

        self.serializer
            .formatter
            .write_before_object_entry(&mut self.serializer.output)
            .map_err(Self::Error::io)?;

        self.serializer
            .formatter
            .write_before_object_key(&mut self.serializer.output)
            .map_err(Self::Error::io)?;

        key.serialize(&mut *self.serializer)?;

        self.serializer
            .formatter
            .write_after_object_key(&mut self.serializer.output)
            .map_err(Self::Error::io)?;

        self.serializer
            .formatter
            .write_before_object_value(&mut self.serializer.output)
            .map_err(Self::Error::io)?;

        value.serialize(&mut *self.serializer)?;

        self.serializer
            .formatter
            .write_after_object_value(&mut self.serializer.output)
            .map_err(Self::Error::io)?;

        self.serializer
            .formatter
            .write_after_object_entry(&mut self.serializer.output)
            .map_err(Self::Error::io)
    }

    fn end(self) -> Result<Self::Ok> {
        self.serializer
            .formatter
            .write_object_end(&mut self.serializer.output)
            .map_err(Self::Error::io)
    }
}
