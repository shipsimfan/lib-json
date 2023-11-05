use super::Formatter;
use crate::{Error, Result};
use std::io::Write;

pub(super) struct MapSerializer<'a, W: Write, F: Formatter> {
    output: &'a mut W,
    formatter: &'a mut F,
}

impl<'a, W: Write, F: Formatter> MapSerializer<'a, W, F> {
    pub(super) fn new(output: &'a mut W, formatter: &'a mut F, len: Option<usize>) -> Result<Self> {
        formatter
            .write_object_begin(output, len)
            .map(|_| MapSerializer { output, formatter })
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
        todo!("Serialize entry")
    }

    fn end(mut self) -> Result<Self::Ok> {
        self.formatter
            .write_object_end(&mut self.output)
            .map_err(Error::io)
    }
}
