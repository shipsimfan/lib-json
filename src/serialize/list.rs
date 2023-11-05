use super::{Formatter, Serializer};
use crate::{Error, Result};
use std::io::Write;

pub(super) struct ListSerializer<'a, W: Write, F: Formatter> {
    serializer: &'a mut Serializer<W, F>,
}

impl<'a, W: Write, F: Formatter> ListSerializer<'a, W, F> {
    pub(super) fn new(serializer: &'a mut Serializer<W, F>, len: Option<usize>) -> Result<Self> {
        serializer
            .formatter
            .write_array_begin(&mut serializer.output, len)
            .map(|_| ListSerializer { serializer })
            .map_err(Error::io)
    }
}

impl<'a, W: Write, F: Formatter> data_format::ListSerializer for ListSerializer<'a, W, F> {
    type Ok = ();
    type Error = Error;

    fn serialize_item<T: data_format::Serialize + ?Sized>(&mut self, item: &T) -> Result<()> {
        self.serializer
            .formatter
            .write_before_array_item(&mut self.serializer.output)
            .map_err(Error::io)?;

        item.serialize(&mut *self.serializer)?;

        self.serializer
            .formatter
            .write_after_array_item(&mut self.serializer.output)
            .map_err(Error::io)
    }

    fn end(self) -> Result<Self::Ok> {
        self.serializer
            .formatter
            .write_array_end(&mut self.serializer.output)
            .map_err(Error::io)
    }
}
