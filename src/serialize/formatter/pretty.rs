use super::Formatter;
use std::io::{Result, Write};

pub(in crate::serialize) struct PrettyFormatter {}

impl PrettyFormatter {
    pub(in crate::serialize) const fn new() -> Self {
        PrettyFormatter {}
    }
}

impl Formatter for PrettyFormatter {
    fn write_begin_array<W: Write + ?Sized>(
        &mut self,
        output: &mut W,
        _: Option<usize>,
    ) -> Result<()> {
        todo!("Pretty being array")
    }

    fn write_begin_object<W: Write + ?Sized>(
        &mut self,
        output: &mut W,
        _: Option<usize>,
    ) -> Result<()> {
        todo!("Pretty begin object")
    }
}
