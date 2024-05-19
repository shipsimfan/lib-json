use super::{ListDeserializer, MapDeserializer, Stream};
use crate::DeserializeError;
use data_format::Converter;

/// A structure which deserializes JSON from a stream of bytes
pub(super) struct Deserializer<'a, 'de> {
    /// The stream to read bytes from
    stream: &'a mut Stream<'de>,
}

impl<'a, 'de> Deserializer<'a, 'de> {
    /// Creates a new [`Deserializer`] over `stream`
    pub(super) fn new(stream: &'a mut Stream<'de>) -> Self {
        Deserializer { stream }
    }
}

impl<'a, 'de> data_format::Deserializer<'de> for Deserializer<'a, 'de> {
    type Error = DeserializeError<'de>;

    fn deserialize_any<C: Converter<'de>>(self, converter: C) -> Result<C::Value, Self::Error> {
        self.stream.skip_whitespace();
        match self.stream.peek().ok_or(Self::Error::UnexpectedEndOfJSON)? {
            b't' | b'f' => self.deserialize_bool(converter),
            b'n' => self.deserialize_unit(converter),
            c if c.is_ascii_digit() => self.deserialize_f64(converter),
            b'-' => self.deserialize_f64(converter),
            b'\"' => self.deserialize_string(converter),
            b'[' => self.deserialize_list(converter),
            b'{' => self.deserialize_map(converter),
            _ => Err(Self::Error::Unexpected {
                unexpected: self.stream.get_next_byte(),
                expected: "valid JSON",
            }),
        }
    }

    fn deserialize_bool<C: Converter<'de>>(self, converter: C) -> Result<C::Value, Self::Error> {
        self.stream.skip_whitespace();
        let value = match self.stream.peek().ok_or(Self::Error::UnexpectedEndOfJSON)? {
            b'f' => self.stream.expect_str("false").map(|_| false),
            b't' => self.stream.expect_str("true").map(|_| true),
            _ => Err(Self::Error::Unexpected {
                unexpected: self.stream.get_next_byte(),
                expected: "true or false",
            }),
        }?;
        converter.convert_bool(value)
    }

    fn deserialize_i8<C: Converter<'de>>(self, converter: C) -> Result<C::Value, Self::Error> {
        self.deserialize_isize(converter)
    }

    fn deserialize_i16<C: Converter<'de>>(self, converter: C) -> Result<C::Value, Self::Error> {
        self.deserialize_isize(converter)
    }

    fn deserialize_i32<C: Converter<'de>>(self, converter: C) -> Result<C::Value, Self::Error> {
        self.deserialize_isize(converter)
    }

    fn deserialize_i64<C: Converter<'de>>(self, converter: C) -> Result<C::Value, Self::Error> {
        self.deserialize_isize(converter)
    }

    fn deserialize_i128<C: Converter<'de>>(self, converter: C) -> Result<C::Value, Self::Error> {
        self.deserialize_isize(converter)
    }

    fn deserialize_isize<C: Converter<'de>>(self, converter: C) -> Result<C::Value, Self::Error> {
        let value = super::number::deserialize_isize(self.stream)?;
        converter.convert_isize(value)
    }

    fn deserialize_u8<C: Converter<'de>>(self, converter: C) -> Result<C::Value, Self::Error> {
        self.deserialize_usize(converter)
    }

    fn deserialize_u16<C: Converter<'de>>(self, converter: C) -> Result<C::Value, Self::Error> {
        self.deserialize_usize(converter)
    }

    fn deserialize_u32<C: Converter<'de>>(self, converter: C) -> Result<C::Value, Self::Error> {
        self.deserialize_usize(converter)
    }

    fn deserialize_u64<C: Converter<'de>>(self, converter: C) -> Result<C::Value, Self::Error> {
        self.deserialize_usize(converter)
    }

    fn deserialize_u128<C: Converter<'de>>(self, converter: C) -> Result<C::Value, Self::Error> {
        self.deserialize_usize(converter)
    }

    fn deserialize_usize<C: Converter<'de>>(self, converter: C) -> Result<C::Value, Self::Error> {
        let value = super::number::deserialize_usize(self.stream)?;
        converter.convert_usize(value)
    }

    fn deserialize_f32<C: Converter<'de>>(self, converter: C) -> Result<C::Value, Self::Error> {
        self.deserialize_f64(converter)
    }

    fn deserialize_f64<C: Converter<'de>>(self, converter: C) -> Result<C::Value, Self::Error> {
        let value = super::number::deserialize_f64(self.stream)?;
        converter.convert_f64(value)
    }

    fn deserialize_string<C: Converter<'de>>(self, converter: C) -> Result<C::Value, Self::Error> {
        let string = super::string::deserialize_string(self.stream)?;
        converter.convert_str_borrow(string)
    }

    fn deserialize_unit<C: Converter<'de>>(self, converter: C) -> Result<C::Value, Self::Error> {
        self.stream.expect_str("null")?;
        converter.convert_unit()
    }

    fn deserialize_list<C: Converter<'de>>(self, converter: C) -> Result<C::Value, Self::Error> {
        self.stream.skip_whitespace();

        let start_index = self.stream.index();
        self.stream.expect(b'[', start_index, "'['")?;

        let result = converter.convert_list(ListDeserializer::new(self.stream, start_index))?;

        self.stream.expect(b']', start_index, "']'")?;
        Ok(result)
    }

    fn deserialize_map<C: Converter<'de>>(self, converter: C) -> Result<C::Value, Self::Error> {
        self.stream.skip_whitespace();

        let start_index = self.stream.index();
        self.stream.expect(b'{', start_index, "'{'")?;

        let result = converter.convert_map(MapDeserializer::new(self.stream, start_index))?;

        self.stream.expect(b'}', start_index, "'}'")?;
        Ok(result)
    }
}
