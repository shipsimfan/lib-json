use crate::{
    deserialize::{
        expect, number, peek, skip_whitespace, string, Deserializer, ListDeserializer,
        MapDeserializer,
    },
    DeserializeError,
};
use data_format::Converter;

impl<'a, 'de> data_format::Deserializer<'de> for Deserializer<'a, 'de> {
    type Error = DeserializeError<'de>;

    fn deserialize_any<C: Converter<'de>>(mut self, converter: C) -> Result<C::Value, Self::Error> {
        skip_whitespace(&mut self.stream)?;
        match peek(&mut self.stream)? {
            ('t', _) | ('f', _) => self.deserialize_bool(converter),
            ('n', _) => self.deserialize_unit(converter),
            ('-', _) => self.deserialize_f64(converter),
            ('\"', _) => self.deserialize_string(converter),
            ('[', _) => self.deserialize_list(converter),
            ('{', _) => self.deserialize_map(converter),
            (c, _) if c.is_ascii_digit() => self.deserialize_f64(converter),
            (c, pos) => Err(DeserializeError::unexpected(c, "valid JSON", pos)),
        }
    }

    fn deserialize_bool<C: Converter<'de>>(
        mut self,
        converter: C,
    ) -> Result<C::Value, Self::Error> {
        skip_whitespace(&mut self.stream)?;
        let (value, pos) = match peek(&mut self.stream)? {
            ('f', pos) => expect(&mut self.stream, "false", "false").map(|_| (false, pos)),
            ('t', pos) => expect(&mut self.stream, "true", "true").map(|_| (true, pos)),
            (c, pos) => Err(DeserializeError::unexpected(c, "true or false", pos)),
        }?;
        converter
            .convert_bool(value)
            .map_err(|mut error: DeserializeError<'de>| {
                error.set_position(pos);
                error
            })
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
        let (value, pos) = number::deserialize_isize(self.stream)?;
        converter
            .convert_isize(value)
            .map_err(|mut error: DeserializeError<'de>| {
                error.set_position(pos);
                error
            })
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
        let (value, pos) = number::deserialize_usize(self.stream)?;
        converter
            .convert_usize(value)
            .map_err(|mut error: DeserializeError<'de>| {
                error.set_position(pos);
                error
            })
    }

    fn deserialize_f32<C: Converter<'de>>(self, converter: C) -> Result<C::Value, Self::Error> {
        self.deserialize_f64(converter)
    }

    fn deserialize_f64<C: Converter<'de>>(self, converter: C) -> Result<C::Value, Self::Error> {
        let (value, pos) = number::deserialize_f64(self.stream)?;
        converter
            .convert_f64(value)
            .map_err(|mut error: DeserializeError<'de>| {
                error.set_position(pos);
                error
            })
    }

    fn deserialize_string<C: Converter<'de>>(self, converter: C) -> Result<C::Value, Self::Error> {
        let (string, pos) = string::deserialize_string(self.stream)?;
        converter
            .convert_str_borrow(string)
            .map_err(|mut error: DeserializeError<'de>| {
                error.set_position(pos);
                error
            })
    }

    fn deserialize_unit<C: Converter<'de>>(
        mut self,
        converter: C,
    ) -> Result<C::Value, Self::Error> {
        let pos = self.stream.position();
        expect(&mut self.stream, "null", "null")?;
        converter
            .convert_unit()
            .map_err(|mut error: DeserializeError<'de>| {
                error.set_position(pos);
                error
            })
    }

    fn deserialize_list<C: Converter<'de>>(
        mut self,
        converter: C,
    ) -> Result<C::Value, Self::Error> {
        skip_whitespace(&mut self.stream)?;

        expect(&mut self.stream, '[', "a list")?;

        let pos = self.stream.position();
        let result = converter
            .convert_list(ListDeserializer::new(self.stream))
            .map_err(|mut error| {
                error.set_position(pos);
                error
            })?;

        expect(&mut self.stream, ']', "']'")?;
        Ok(result)
    }

    fn deserialize_map<C: Converter<'de>>(mut self, converter: C) -> Result<C::Value, Self::Error> {
        skip_whitespace(&mut self.stream)?;

        expect(&mut self.stream, '{', "a map")?;

        let pos = self.stream.position();
        let result = converter
            .convert_map(MapDeserializer::new(self.stream))
            .map_err(|mut error| {
                error.set_position(pos);
                error
            })?;

        expect(&mut self.stream, '}', "'}'")?;
        Ok(result)
    }
}
