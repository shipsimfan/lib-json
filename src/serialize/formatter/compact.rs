use super::Formatter;
use std::io::{Result, Write};

pub(in crate::serialize) struct CompactFormatter {
    first: bool,
}

impl CompactFormatter {
    pub(in crate::serialize) const fn new() -> Self {
        CompactFormatter { first: true }
    }
}

impl Formatter for CompactFormatter {
    fn write_array_begin<W: Write + ?Sized>(
        &mut self,
        output: &mut W,
        _: Option<usize>,
    ) -> Result<()> {
        self.first = true;
        output.write_all(b"[")
    }

    fn write_before_array_item<W: Write + ?Sized>(&mut self, output: &mut W) -> Result<()> {
        if self.first {
            self.first = false;
            Ok(())
        } else {
            output.write_all(b",")
        }
    }

    fn write_after_array_item<W: Write + ?Sized>(&mut self, _: &mut W) -> Result<()> {
        Ok(())
    }

    fn write_array_end<W: Write + ?Sized>(&mut self, output: &mut W) -> Result<()> {
        self.first = false;
        output.write_all(b"]")
    }

    fn write_object_begin<W: Write + ?Sized>(
        &mut self,
        output: &mut W,
        _: Option<usize>,
    ) -> Result<()> {
        self.first = true;
        output.write_all(b"{")
    }

    fn write_before_object_entry<W: Write + ?Sized>(&mut self, output: &mut W) -> Result<()> {
        if self.first {
            self.first = false;
            Ok(())
        } else {
            output.write_all(b",")
        }
    }

    fn write_after_object_entry<W: Write + ?Sized>(&mut self, _: &mut W) -> Result<()> {
        Ok(())
    }

    fn write_before_object_key<W: Write + ?Sized>(&mut self, _: &mut W) -> Result<()> {
        Ok(())
    }

    fn write_after_object_key<W: Write + ?Sized>(&mut self, output: &mut W) -> Result<()> {
        output.write_all(b":")
    }

    fn write_object_end<W: Write + ?Sized>(&mut self, output: &mut W) -> Result<()> {
        self.first = false;
        output.write_all(b"}")
    }
}
