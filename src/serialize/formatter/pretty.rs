use super::Formatter;
use std::io::{Result, Write};

pub(in crate::serialize) struct PrettyFormatter {
    depth: usize,
    first: bool,
}

impl PrettyFormatter {
    pub(in crate::serialize) const fn new() -> Self {
        PrettyFormatter {
            depth: 0,
            first: true,
        }
    }

    fn write_prefix<W: Write + ?Sized>(&mut self, output: &mut W) -> Result<()> {
        output.write_all(b"\n")?;
        for _ in 0..self.depth {
            output.write_all(b"    ")?;
        }
        Ok(())
    }
}

impl Formatter for PrettyFormatter {
    fn write_array_begin<W: Write + ?Sized>(
        &mut self,
        output: &mut W,
        _: Option<usize>,
    ) -> Result<()> {
        self.first = true;
        self.depth += 1;
        output.write_all(b"[")
    }

    fn write_array_end<W: Write + ?Sized>(&mut self, output: &mut W) -> Result<()> {
        self.depth -= 1;

        if !self.first {
            self.write_prefix(output)?;
        }

        self.first = false;
        output.write_all(b"}")
    }

    fn write_object_begin<W: Write + ?Sized>(
        &mut self,
        output: &mut W,
        _: Option<usize>,
    ) -> Result<()> {
        self.first = true;
        self.depth += 1;
        output.write_all(b"{")
    }

    fn write_object_end<W: Write + ?Sized>(&mut self, output: &mut W) -> Result<()> {
        self.depth -= 1;

        if !self.first {
            self.write_prefix(output)?;
        }

        self.first = false;
        output.write_all(b"}")
    }
}
