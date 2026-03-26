use super::Escape;
#[cfg(feature = "no_std")]
use core::fmt::{Error, Write};
#[cfg(not(feature = "no_std"))]
use std::io::{Error, Write};

mod compact;
mod pretty;

pub(super) use compact::CompactFormatter;
pub(super) use pretty::PrettyFormatter;

pub(super) trait Formatter {
    fn write_null<W: Write + ?Sized>(&mut self, output: &mut W) -> Result<(), Error> {
        write!(output, "null")
    }

    fn write_bool<W: Write + ?Sized>(&mut self, output: &mut W, value: bool) -> Result<(), Error> {
        write!(output, "{}", if value { "true" } else { "false" })
    }

    fn write_i8<W: Write + ?Sized>(&mut self, output: &mut W, value: i8) -> Result<(), Error> {
        write!(output, "{}", value)
    }

    fn write_i16<W: Write + ?Sized>(&mut self, output: &mut W, value: i16) -> Result<(), Error> {
        write!(output, "{}", value)
    }

    fn write_i32<W: Write + ?Sized>(&mut self, output: &mut W, value: i32) -> Result<(), Error> {
        write!(output, "{}", value)
    }

    fn write_i64<W: Write + ?Sized>(&mut self, output: &mut W, value: i64) -> Result<(), Error> {
        write!(output, "{}", value)
    }

    fn write_i128<W: Write + ?Sized>(&mut self, output: &mut W, value: i128) -> Result<(), Error> {
        write!(output, "{}", value)
    }

    fn write_u8<W: Write + ?Sized>(&mut self, output: &mut W, value: u8) -> Result<(), Error> {
        write!(output, "{}", value)
    }

    fn write_u16<W: Write + ?Sized>(&mut self, output: &mut W, value: u16) -> Result<(), Error> {
        write!(output, "{}", value)
    }

    fn write_u32<W: Write + ?Sized>(&mut self, output: &mut W, value: u32) -> Result<(), Error> {
        write!(output, "{}", value)
    }

    fn write_u64<W: Write + ?Sized>(&mut self, output: &mut W, value: u64) -> Result<(), Error> {
        write!(output, "{}", value)
    }

    fn write_u128<W: Write + ?Sized>(&mut self, output: &mut W, value: u128) -> Result<(), Error> {
        write!(output, "{}", value)
    }

    fn write_f32<W: Write + ?Sized>(&mut self, output: &mut W, value: f32) -> Result<(), Error> {
        write!(output, "{}", value)
    }

    fn write_f64<W: Write + ?Sized>(&mut self, output: &mut W, value: f64) -> Result<(), Error> {
        write!(output, "{}", value)
    }

    fn write_str_begin<W: Write + ?Sized>(&mut self, output: &mut W) -> Result<(), Error> {
        write!(output, "\"")
    }

    fn write_str<W: Write + ?Sized>(&mut self, output: &mut W, str: &str) -> Result<(), Error> {
        write!(output, "{}", str)
    }

    fn write_str_escape_char<W: Write + ?Sized>(
        &mut self,
        output: &mut W,
        escape: Escape,
    ) -> Result<(), Error> {
        escape.write(output)
    }

    fn write_str_end<W: Write + ?Sized>(&mut self, output: &mut W) -> Result<(), Error> {
        write!(output, "\"")
    }

    fn write_array_begin<W: Write + ?Sized>(
        &mut self,
        output: &mut W,
        len: Option<usize>,
    ) -> Result<(), Error>;

    fn write_before_array_item<W: Write + ?Sized>(&mut self, output: &mut W) -> Result<(), Error>;
    fn write_after_array_item<W: Write + ?Sized>(&mut self, output: &mut W) -> Result<(), Error>;

    fn write_array_end<W: Write + ?Sized>(&mut self, output: &mut W) -> Result<(), Error>;

    fn write_object_begin<W: Write + ?Sized>(
        &mut self,
        output: &mut W,
        len: Option<usize>,
    ) -> Result<(), Error>;

    fn write_before_object_entry<W: Write + ?Sized>(&mut self, output: &mut W)
        -> Result<(), Error>;
    fn write_after_object_entry<W: Write + ?Sized>(&mut self, output: &mut W) -> Result<(), Error>;

    fn write_before_object_key<W: Write + ?Sized>(&mut self, output: &mut W) -> Result<(), Error>;
    fn write_after_object_key<W: Write + ?Sized>(&mut self, output: &mut W) -> Result<(), Error>;

    #[allow(unused_variables)]
    fn write_before_object_value<W: Write + ?Sized>(
        &mut self,
        output: &mut W,
    ) -> Result<(), Error> {
        Ok(())
    }

    #[allow(unused_variables)]
    fn write_after_object_value<W: Write + ?Sized>(&mut self, output: &mut W) -> Result<(), Error> {
        Ok(())
    }

    fn write_object_end<W: Write + ?Sized>(&mut self, output: &mut W) -> Result<(), Error>;
}
