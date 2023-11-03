use super::{CompactFormatter, Formatter, ListSerializer, MapSerializer, PrettyFormatter};
use crate::Error;
use std::io::Write;

pub(super) struct Serializer<W: Write, F: Formatter> {
    output: W,
    formatter: F,
}

impl<W: Write> Serializer<W, CompactFormatter> {
    pub(super) fn compact(output: W) -> Self {
        Self::with_formatter(output, CompactFormatter::new())
    }
}

impl<W: Write> Serializer<W, PrettyFormatter> {
    pub(super) fn pretty(output: W) -> Self {
        Self::with_formatter(output, PrettyFormatter::new())
    }
}

impl<W: Write, F: Formatter> Serializer<W, F> {
    pub(super) fn with_formatter(output: W, formatter: F) -> Self {
        Serializer { output, formatter }
    }
}

impl<'a, W: Write, F: Formatter> data_format::Serializer for &'a mut Serializer<W, F> {
    type Ok = ();
    type Error = Error;

    type ListSerializer = ListSerializer<'a, W, F>;
    type MapSerializer = MapSerializer<'a, W, F>;

    fn serialize_bool(self, value: bool) -> Result<Self::Ok, Self::Error> {
        self.formatter
            .write_bool(&mut self.output, value)
            .map_err(Error::io)
    }

    fn serialize_i8(self, value: i8) -> Result<Self::Ok, Self::Error> {
        self.formatter
            .write_i8(&mut self.output, value)
            .map_err(Error::io)
    }

    fn serialize_i16(self, value: i16) -> Result<Self::Ok, Self::Error> {
        self.formatter
            .write_i16(&mut self.output, value)
            .map_err(Error::io)
    }

    fn serialize_i32(self, value: i32) -> Result<Self::Ok, Self::Error> {
        self.formatter
            .write_i32(&mut self.output, value)
            .map_err(Error::io)
    }

    fn serialize_i64(self, value: i64) -> Result<Self::Ok, Self::Error> {
        self.formatter
            .write_i64(&mut self.output, value)
            .map_err(Error::io)
    }

    fn serialize_i128(self, value: i128) -> Result<Self::Ok, Self::Error> {
        self.formatter
            .write_i128(&mut self.output, value)
            .map_err(Error::io)
    }

    fn serialize_u8(self, value: u8) -> Result<Self::Ok, Self::Error> {
        self.formatter
            .write_u8(&mut self.output, value)
            .map_err(Error::io)
    }

    fn serialize_u16(self, value: u16) -> Result<Self::Ok, Self::Error> {
        self.formatter
            .write_u16(&mut self.output, value)
            .map_err(Error::io)
    }

    fn serialize_u32(self, value: u32) -> Result<Self::Ok, Self::Error> {
        self.formatter
            .write_u32(&mut self.output, value)
            .map_err(Error::io)
    }

    fn serialize_u64(self, value: u64) -> Result<Self::Ok, Self::Error> {
        self.formatter
            .write_u64(&mut self.output, value)
            .map_err(Error::io)
    }

    fn serialize_u128(self, value: u128) -> Result<Self::Ok, Self::Error> {
        self.formatter
            .write_u128(&mut self.output, value)
            .map_err(Error::io)
    }

    fn serialize_f32(self, value: f32) -> Result<Self::Ok, Self::Error> {
        self.formatter
            .write_f32(&mut self.output, value)
            .map_err(Error::io)
    }

    fn serialize_f64(self, value: f64) -> Result<Self::Ok, Self::Error> {
        self.formatter
            .write_f64(&mut self.output, value)
            .map_err(Error::io)
    }

    fn serialize_string(self, value: &str) -> Result<Self::Ok, Self::Error> {
        todo!("Handle escaping the string")
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        self.formatter
            .write_null(&mut self.output)
            .map_err(Error::io)
    }

    fn serialize_list(self, len: Option<usize>) -> Result<Self::ListSerializer, Self::Error> {
        ListSerializer::new(&mut self.output, &mut self.formatter, len)
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::MapSerializer, Self::Error> {
        MapSerializer::new(&mut self.output, &mut self.formatter, len)
    }
}
