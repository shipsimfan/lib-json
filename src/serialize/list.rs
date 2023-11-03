use super::Formatter;
use crate::{Error, Result};
use std::io::Write;

pub(super) struct ListSerializer<'a, W: Write, F: Formatter> {
    output: &'a mut W,
    formatter: &'a mut F,
}

impl<'a, W: Write, F: Formatter> ListSerializer<'a, W, F> {
    pub(super) fn new(output: &'a mut W, formatter: &'a mut F, len: Option<usize>) -> Result<Self> {
        formatter
            .write_begin_array(output, len)
            .map(|_| ListSerializer { output, formatter })
            .map_err(Error::io)
    }
}

impl<'a, W: Write, F: Formatter> data_format::ListSerializer for ListSerializer<'a, W, F> {
    type Ok = ();
    type Error = Error;

    fn serialize_item<T: data_format::Serialize + ?Sized>(&mut self, item: &T) -> Result<()> {
        todo!("Serialize item")
    }

    fn end(self) -> Result<Self::Ok> {
        todo!("List end")
    }
}
