use super::{Formatter, Result, Serializer};
use crate::SerializeError;
use std::io::Write;

/// Serializes lists into JSON using a [`Formatter`]
pub(super) struct ListSerializer<'a, W: Write, F: Formatter> {
    serializer: &'a mut Serializer<W, F>,
}

impl<'a, W: Write, F: Formatter> ListSerializer<'a, W, F> {
    /// Creates a new [`ListSerializer`] with a length hint `len`
    pub(super) fn new(serializer: &'a mut Serializer<W, F>, len: Option<usize>) -> Result<Self> {
        serializer
            .formatter
            .write_array_begin(&mut serializer.output, len)
            .map(|_| ListSerializer { serializer })
            .map_err(SerializeError::io)
    }
}

impl<'a, W: Write, F: Formatter> data_format::ListSerializer for ListSerializer<'a, W, F> {
    type Ok = ();
    type Error = SerializeError;

    fn serialize_item<T: data_format::Serialize + ?Sized>(&mut self, item: &T) -> Result<()> {
        self.serializer
            .formatter
            .write_before_array_item(&mut self.serializer.output)
            .map_err(Self::Error::io)?;

        item.serialize(&mut *self.serializer)?;

        self.serializer
            .formatter
            .write_after_array_item(&mut self.serializer.output)
            .map_err(Self::Error::io)
    }

    fn end(self) -> Result<Self::Ok> {
        self.serializer
            .formatter
            .write_array_end(&mut self.serializer.output)
            .map_err(Self::Error::io)
    }
}
