use super::Formatter;
#[cfg(feature = "no_std")]
use core::fmt::{Error, Write};
#[cfg(not(feature = "no_std"))]
use std::io::{Error, Write};

/// A [`Formatter`] which outputs JSON with minimal spacing
pub(in crate::serialize) struct CompactFormatter {
    first: bool,
}

impl CompactFormatter {
    /// Creates a new [`CompactFormatter`]
    pub(in crate::serialize) const fn new() -> Self {
        CompactFormatter { first: true }
    }
}

impl Formatter for CompactFormatter {
    fn write_array_begin<W: Write + ?Sized>(
        &mut self,
        output: &mut W,
        _: Option<usize>,
    ) -> Result<(), Error> {
        self.first = true;
        write!(output, "[")
    }

    fn write_before_array_item<W: Write + ?Sized>(&mut self, output: &mut W) -> Result<(), Error> {
        if self.first {
            self.first = false;
            Ok(())
        } else {
            write!(output, ",")
        }
    }

    fn write_after_array_item<W: Write + ?Sized>(&mut self, _: &mut W) -> Result<(), Error> {
        Ok(())
    }

    fn write_array_end<W: Write + ?Sized>(&mut self, output: &mut W) -> Result<(), Error> {
        self.first = false;
        write!(output, "]")
    }

    fn write_object_begin<W: Write + ?Sized>(
        &mut self,
        output: &mut W,
        _: Option<usize>,
    ) -> Result<(), Error> {
        self.first = true;
        write!(output, "{{")
    }

    fn write_before_object_entry<W: Write + ?Sized>(
        &mut self,
        output: &mut W,
    ) -> Result<(), Error> {
        if self.first {
            self.first = false;
            Ok(())
        } else {
            write!(output, ",")
        }
    }

    fn write_after_object_entry<W: Write + ?Sized>(&mut self, _: &mut W) -> Result<(), Error> {
        Ok(())
    }

    fn write_before_object_key<W: Write + ?Sized>(&mut self, _: &mut W) -> Result<(), Error> {
        Ok(())
    }

    fn write_after_object_key<W: Write + ?Sized>(&mut self, output: &mut W) -> Result<(), Error> {
        write!(output, ":")
    }

    fn write_object_end<W: Write + ?Sized>(&mut self, output: &mut W) -> Result<(), Error> {
        self.first = false;
        write!(output, "}}")
    }
}
