use super::{CompactFormatter, Formatter, PrettyFormatter};
use crate::Error;
use std::io::Write;

enum ComplexClass {
    Array,
    Object,
}

pub(super) struct Serializer<W: Write, F: Formatter> {
    output: W,
    formatter: F,
}

pub(super) struct Complex<'a, W: 'a + Write, F: 'a + Formatter> {
    serializer: &'a mut Serializer<W, F>,
    class: ComplexClass,
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

    type ListSerializer = Complex<'a, W, F>;
    type MapSerializer = Complex<'a, W, F>;

    fn serialize_bool(self, value: bool) -> Result<Self::Ok, Self::Error> {
        self.output
            .write_all(if value { b"true" } else { b"false" })
            .map_err(|error| Error::IO(error))
    }

    fn serialize_i8(self, value: i8) -> Result<Self::Ok, Self::Error> {
        write!(self.output, "{}", value).map_err(|error| Error::IO(error))
    }

    fn serialize_i16(self, value: i16) -> Result<Self::Ok, Self::Error> {
        write!(self.output, "{}", value).map_err(|error| Error::IO(error))
    }

    fn serialize_i32(self, value: i32) -> Result<Self::Ok, Self::Error> {
        write!(self.output, "{}", value).map_err(|error| Error::IO(error))
    }

    fn serialize_i64(self, value: i64) -> Result<Self::Ok, Self::Error> {
        write!(self.output, "{}", value).map_err(|error| Error::IO(error))
    }

    fn serialize_i128(self, value: i128) -> Result<Self::Ok, Self::Error> {
        write!(self.output, "{}", value).map_err(|error| Error::IO(error))
    }

    fn serialize_u8(self, value: u8) -> Result<Self::Ok, Self::Error> {
        write!(self.output, "{}", value).map_err(|error| Error::IO(error))
    }

    fn serialize_u16(self, value: u16) -> Result<Self::Ok, Self::Error> {
        write!(self.output, "{}", value).map_err(|error| Error::IO(error))
    }

    fn serialize_u32(self, value: u32) -> Result<Self::Ok, Self::Error> {
        write!(self.output, "{}", value).map_err(|error| Error::IO(error))
    }

    fn serialize_u64(self, value: u64) -> Result<Self::Ok, Self::Error> {
        write!(self.output, "{}", value).map_err(|error| Error::IO(error))
    }

    fn serialize_u128(self, value: u128) -> Result<Self::Ok, Self::Error> {
        write!(self.output, "{}", value).map_err(|error| Error::IO(error))
    }

    fn serialize_f32(self, value: f32) -> Result<Self::Ok, Self::Error> {
        write!(self.output, "{}", value).map_err(|error| Error::IO(error))
    }

    fn serialize_f64(self, value: f64) -> Result<Self::Ok, Self::Error> {
        write!(self.output, "{}", value).map_err(|error| Error::IO(error))
    }

    fn serialize_string(self, value: &str) -> Result<Self::Ok, Self::Error> {
        todo!("Handle escaping the string")
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        self.output
            .write_all(b"null")
            .map_err(|error| Error::IO(error))
    }

    fn serialize_list(self, len: Option<usize>) -> Result<Self::ListSerializer, Self::Error> {
        todo!()
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::MapSerializer, Self::Error> {
        todo!()
    }
}

impl<'a, W: Write, F: Formatter> data_format::ListSerializer for Complex<'a, W, F> {
    type Ok = ();
    type Error = Error;

    fn serialize_item<T: data_format::Serialize + ?Sized>(
        &mut self,
        item: &T,
    ) -> Result<(), Self::Error> {
        todo!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
}

impl<'a, W: Write, F: Formatter> data_format::MapSerializer for Complex<'a, W, F> {
    type Ok = ();
    type Error = Error;

    fn serialize_entry<V: data_format::Serialize + ?Sized>(
        &mut self,
        key: &str,
        value: &V,
    ) -> Result<(), Self::Error> {
        todo!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
}
