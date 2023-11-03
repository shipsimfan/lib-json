use std::io::{Result, Write};

mod compact;
mod pretty;

pub(super) use compact::CompactFormatter;
pub(super) use pretty::PrettyFormatter;

pub(super) trait Formatter {
    fn write_null<W: Write + ?Sized>(&mut self, output: &mut W) -> Result<()> {
        output.write_all(b"null")
    }

    fn write_bool<W: Write + ?Sized>(&mut self, output: &mut W, value: bool) -> Result<()> {
        output.write_all(if value { b"true" } else { b"false" })
    }

    fn write_i8<W: Write + ?Sized>(&mut self, output: &mut W, value: i8) -> Result<()> {
        write!(output, "{}", value)
    }

    fn write_i16<W: Write + ?Sized>(&mut self, output: &mut W, value: i16) -> Result<()> {
        write!(output, "{}", value)
    }

    fn write_i32<W: Write + ?Sized>(&mut self, output: &mut W, value: i32) -> Result<()> {
        write!(output, "{}", value)
    }

    fn write_i64<W: Write + ?Sized>(&mut self, output: &mut W, value: i64) -> Result<()> {
        write!(output, "{}", value)
    }

    fn write_i128<W: Write + ?Sized>(&mut self, output: &mut W, value: i128) -> Result<()> {
        write!(output, "{}", value)
    }

    fn write_isize<W: Write + ?Sized>(&mut self, output: &mut W, value: isize) -> Result<()> {
        write!(output, "{}", value)
    }

    fn write_u8<W: Write + ?Sized>(&mut self, output: &mut W, value: u8) -> Result<()> {
        write!(output, "{}", value)
    }

    fn write_u16<W: Write + ?Sized>(&mut self, output: &mut W, value: u16) -> Result<()> {
        write!(output, "{}", value)
    }

    fn write_u32<W: Write + ?Sized>(&mut self, output: &mut W, value: u32) -> Result<()> {
        write!(output, "{}", value)
    }

    fn write_u64<W: Write + ?Sized>(&mut self, output: &mut W, value: u64) -> Result<()> {
        write!(output, "{}", value)
    }

    fn write_u128<W: Write + ?Sized>(&mut self, output: &mut W, value: u128) -> Result<()> {
        write!(output, "{}", value)
    }

    fn write_usize<W: Write + ?Sized>(&mut self, output: &mut W, value: usize) -> Result<()> {
        write!(output, "{}", value)
    }

    fn write_f32<W: Write + ?Sized>(&mut self, output: &mut W, value: f32) -> Result<()> {
        write!(output, "{}", value)
    }

    fn write_f64<W: Write + ?Sized>(&mut self, output: &mut W, value: f64) -> Result<()> {
        write!(output, "{}", value)
    }

    fn write_str<W: Write + ?Sized>(&mut self, output: &mut W, str: &str) -> Result<()> {
        output.write_all(str.as_bytes())
    }

    fn write_escape_char<W: Write + ?Sized>(&mut self, output: &mut W, escape: &str) -> Result<()> {
        output.write_all(&[b'\\'])?;
        output.write_all(escape.as_bytes())
    }

    fn write_begin_array<W: Write + ?Sized>(
        &mut self,
        output: &mut W,
        len: Option<usize>,
    ) -> Result<()>;

    fn write_begin_object<W: Write + ?Sized>(
        &mut self,
        output: &mut W,
        len: Option<usize>,
    ) -> Result<()>;
}
