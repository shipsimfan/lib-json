use data_format::Serialize;

use super::{Formatter, Serializer};
use crate::{Error, Result};
use std::io::Write;

pub(super) struct MapSerializer<'a, W: Write, F: Formatter> {
    serializer: &'a mut Serializer<W, F>,
}

impl<'a, W: Write, F: Formatter> MapSerializer<'a, W, F> {
    pub(super) fn new(serializer: &'a mut Serializer<W, F>, len: Option<usize>) -> Result<Self> {
        serializer
            .formatter
            .write_object_begin(&mut serializer.output, len)
            .map(|_| MapSerializer { serializer })
            .map_err(Error::io)
    }
}

impl<'a, W: Write, F: Formatter> data_format::MapSerializer for MapSerializer<'a, W, F> {
    type Ok = ();
    type Error = Error;

    fn serialize_entry<V: data_format::Serialize + ?Sized>(
        &mut self,
        key: &str,
        value: &V,
    ) -> Result<()> {
        self.serializer
            .formatter
            .write_before_object_entry(&mut self.serializer.output)
            .map_err(Error::io)?;

        self.serializer
            .formatter
            .write_before_object_key(&mut self.serializer.output)
            .map_err(Error::io)?;

        key.serialize(&mut *self.serializer)?;

        self.serializer
            .formatter
            .write_after_object_key(&mut self.serializer.output)
            .map_err(Error::io)?;

        self.serializer
            .formatter
            .write_before_object_value(&mut self.serializer.output)
            .map_err(Error::io)?;

        value.serialize(&mut *self.serializer)?;

        self.serializer
            .formatter
            .write_after_object_value(&mut self.serializer.output)
            .map_err(Error::io)?;

        self.serializer
            .formatter
            .write_after_object_entry(&mut self.serializer.output)
            .map_err(Error::io)
    }

    fn end(self) -> Result<Self::Ok> {
        self.serializer
            .formatter
            .write_object_end(&mut self.serializer.output)
            .map_err(Error::io)
    }
}
