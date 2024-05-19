use super::{CompactFormatter, Escape, Formatter, ListSerializer, MapSerializer, PrettyFormatter};
use crate::SerializeError;
use std::io::Write;

/// A structure which serializes objects into JSON on a [`Write`]
pub(super) struct Serializer<W: Write, F: Formatter> {
    /// The output for the formatted JSON bytes
    pub(super) output: W,

    /// The formatter which determines spacing between elements
    pub(super) formatter: F,
}

impl<W: Write> Serializer<W, CompactFormatter> {
    /// Creates a new [`Serializer`] using a [`CompactFormatter`]
    pub(super) fn compact(output: W) -> Self {
        Self::with_formatter(output, CompactFormatter::new())
    }
}

impl<W: Write> Serializer<W, PrettyFormatter> {
    /// Creates a new [`Serializer`] using a [`PrettyFormatter`]
    pub(super) fn pretty(output: W) -> Self {
        Self::with_formatter(output, PrettyFormatter::new())
    }
}

impl<W: Write, F: Formatter> Serializer<W, F> {
    /// Creates a new [`Serializer`] using `formatter`
    pub(super) fn with_formatter(output: W, formatter: F) -> Self {
        Serializer { output, formatter }
    }
}

impl<'a, W: Write, F: Formatter> data_format::Serializer for &'a mut Serializer<W, F> {
    type Ok = ();
    type Error = SerializeError;

    type ListSerializer = ListSerializer<'a, W, F>;
    type MapSerializer = MapSerializer<'a, W, F>;

    fn serialize_bool(self, value: bool) -> Result<Self::Ok, Self::Error> {
        self.formatter
            .write_bool(&mut self.output, value)
            .map_err(Self::Error::io)
    }

    fn serialize_i8(self, value: i8) -> Result<Self::Ok, Self::Error> {
        self.formatter
            .write_i8(&mut self.output, value)
            .map_err(Self::Error::io)
    }

    fn serialize_i16(self, value: i16) -> Result<Self::Ok, Self::Error> {
        self.formatter
            .write_i16(&mut self.output, value)
            .map_err(Self::Error::io)
    }

    fn serialize_i32(self, value: i32) -> Result<Self::Ok, Self::Error> {
        self.formatter
            .write_i32(&mut self.output, value)
            .map_err(Self::Error::io)
    }

    fn serialize_i64(self, value: i64) -> Result<Self::Ok, Self::Error> {
        self.formatter
            .write_i64(&mut self.output, value)
            .map_err(Self::Error::io)
    }

    fn serialize_i128(self, value: i128) -> Result<Self::Ok, Self::Error> {
        self.formatter
            .write_i128(&mut self.output, value)
            .map_err(Self::Error::io)
    }

    fn serialize_u8(self, value: u8) -> Result<Self::Ok, Self::Error> {
        self.formatter
            .write_u8(&mut self.output, value)
            .map_err(Self::Error::io)
    }

    fn serialize_u16(self, value: u16) -> Result<Self::Ok, Self::Error> {
        self.formatter
            .write_u16(&mut self.output, value)
            .map_err(Self::Error::io)
    }

    fn serialize_u32(self, value: u32) -> Result<Self::Ok, Self::Error> {
        self.formatter
            .write_u32(&mut self.output, value)
            .map_err(Self::Error::io)
    }

    fn serialize_u64(self, value: u64) -> Result<Self::Ok, Self::Error> {
        self.formatter
            .write_u64(&mut self.output, value)
            .map_err(Self::Error::io)
    }

    fn serialize_u128(self, value: u128) -> Result<Self::Ok, Self::Error> {
        self.formatter
            .write_u128(&mut self.output, value)
            .map_err(Self::Error::io)
    }

    fn serialize_f32(self, value: f32) -> Result<Self::Ok, Self::Error> {
        self.formatter
            .write_f32(&mut self.output, value)
            .map_err(Self::Error::io)
    }

    fn serialize_f64(self, value: f64) -> Result<Self::Ok, Self::Error> {
        self.formatter
            .write_f64(&mut self.output, value)
            .map_err(Self::Error::io)
    }

    fn serialize_string(self, value: &str) -> Result<Self::Ok, Self::Error> {
        self.formatter
            .write_str_begin(&mut self.output)
            .map_err(Self::Error::io)?;

        let mut start = 0;
        for char in value.chars() {
            let index = start;
            start += char.len_utf8();
            if let Some(escape) = Escape::from_char(char) {
                self.formatter
                    .write_str_escape_char(&mut self.output, escape)
                    .map_err(Self::Error::io)?;

                continue;
            }

            self.formatter
                .write_str(&mut self.output, &value[index..start])
                .map_err(Self::Error::io)?;
        }

        self.formatter
            .write_str_end(&mut self.output)
            .map_err(Self::Error::io)?;
        Ok(())
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        self.formatter
            .write_null(&mut self.output)
            .map_err(Self::Error::io)
    }

    fn serialize_list(self, len: Option<usize>) -> Result<Self::ListSerializer, Self::Error> {
        ListSerializer::new(self, len)
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::MapSerializer, Self::Error> {
        MapSerializer::new(self, len)
    }
}
