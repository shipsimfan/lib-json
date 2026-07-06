use crate::{
    deserialize::{
        value::{ValueListDeserializer, ValueMapDeserializer},
        ValueDeserializer,
    },
    DeserializeError, Value,
};
use data_format::{Converter, Deserializer};

impl<'de> Deserializer<'de> for ValueDeserializer<'de> {
    type Error = DeserializeError<'de>;

    fn deserialize_any<C: Converter<'de>>(self, converter: C) -> Result<C::Value, Self::Error> {
        match self.value {
            Value::Null => converter.convert_unit(),
            Value::Boolean(boolean) => converter.convert_bool(boolean),
            Value::Number(number) => converter.convert_f64(number),
            Value::String(string) => converter.convert_str_borrow(string),
            Value::Array(list) => converter.convert_list(ValueListDeserializer::new(list)),
            Value::Object(map) => converter.convert_map(ValueMapDeserializer::new(map)),
        }
    }

    fn deserialize_bool<C: Converter<'de>>(self, converter: C) -> Result<C::Value, Self::Error> {
        match self.value {
            Value::Boolean(boolean) => converter.convert_bool(boolean),
            _ => Err(data_format::DeserializeError::invalid_type(
                self.value,
                "a boolean",
            )),
        }
    }

    fn deserialize_i8<C: Converter<'de>>(self, converter: C) -> Result<C::Value, Self::Error> {
        match self.value {
            Value::Number(number) => converter.convert_i128(number as i128),
            _ => Err(data_format::DeserializeError::invalid_type(
                self.value, "a number",
            )),
        }
    }

    fn deserialize_i16<C: Converter<'de>>(self, converter: C) -> Result<C::Value, Self::Error> {
        match self.value {
            Value::Number(number) => converter.convert_i128(number as i128),
            _ => Err(data_format::DeserializeError::invalid_type(
                self.value, "a number",
            )),
        }
    }

    fn deserialize_i32<C: Converter<'de>>(self, converter: C) -> Result<C::Value, Self::Error> {
        match self.value {
            Value::Number(number) => converter.convert_i128(number as i128),
            _ => Err(data_format::DeserializeError::invalid_type(
                self.value, "a number",
            )),
        }
    }

    fn deserialize_i64<C: Converter<'de>>(self, converter: C) -> Result<C::Value, Self::Error> {
        match self.value {
            Value::Number(number) => converter.convert_i128(number as i128),
            _ => Err(data_format::DeserializeError::invalid_type(
                self.value, "a number",
            )),
        }
    }

    fn deserialize_i128<C: Converter<'de>>(self, converter: C) -> Result<C::Value, Self::Error> {
        match self.value {
            Value::Number(number) => converter.convert_i128(number as i128),
            _ => Err(data_format::DeserializeError::invalid_type(
                self.value, "a number",
            )),
        }
    }

    fn deserialize_isize<C: Converter<'de>>(self, converter: C) -> Result<C::Value, Self::Error> {
        match self.value {
            Value::Number(number) => converter.convert_i128(number as i128),
            _ => Err(data_format::DeserializeError::invalid_type(
                self.value, "a number",
            )),
        }
    }

    fn deserialize_u8<C: Converter<'de>>(self, converter: C) -> Result<C::Value, Self::Error> {
        match self.value {
            Value::Number(number) => converter.convert_u128(number as u128),
            _ => Err(data_format::DeserializeError::invalid_type(
                self.value, "a number",
            )),
        }
    }

    fn deserialize_u16<C: Converter<'de>>(self, converter: C) -> Result<C::Value, Self::Error> {
        match self.value {
            Value::Number(number) => converter.convert_u128(number as u128),
            _ => Err(data_format::DeserializeError::invalid_type(
                self.value, "a number",
            )),
        }
    }

    fn deserialize_u32<C: Converter<'de>>(self, converter: C) -> Result<C::Value, Self::Error> {
        match self.value {
            Value::Number(number) => converter.convert_u128(number as u128),
            _ => Err(data_format::DeserializeError::invalid_type(
                self.value, "a number",
            )),
        }
    }

    fn deserialize_u64<C: Converter<'de>>(self, converter: C) -> Result<C::Value, Self::Error> {
        match self.value {
            Value::Number(number) => converter.convert_u128(number as u128),
            _ => Err(data_format::DeserializeError::invalid_type(
                self.value, "a number",
            )),
        }
    }

    fn deserialize_u128<C: Converter<'de>>(self, converter: C) -> Result<C::Value, Self::Error> {
        match self.value {
            Value::Number(number) => converter.convert_u128(number as u128),
            _ => Err(data_format::DeserializeError::invalid_type(
                self.value, "a number",
            )),
        }
    }

    fn deserialize_usize<C: Converter<'de>>(self, converter: C) -> Result<C::Value, Self::Error> {
        match self.value {
            Value::Number(number) => converter.convert_u128(number as u128),
            _ => Err(data_format::DeserializeError::invalid_type(
                self.value, "a number",
            )),
        }
    }

    fn deserialize_f32<C: Converter<'de>>(self, converter: C) -> Result<C::Value, Self::Error> {
        match self.value {
            Value::Number(number) => converter.convert_f64(number),
            _ => Err(data_format::DeserializeError::invalid_type(
                self.value, "a number",
            )),
        }
    }

    fn deserialize_f64<C: Converter<'de>>(self, converter: C) -> Result<C::Value, Self::Error> {
        match self.value {
            Value::Number(number) => converter.convert_f64(number),
            _ => Err(data_format::DeserializeError::invalid_type(
                self.value, "a number",
            )),
        }
    }

    fn deserialize_string<C: Converter<'de>>(self, converter: C) -> Result<C::Value, Self::Error> {
        match self.value {
            Value::String(string) => converter.convert_str_borrow(string),
            _ => Err(data_format::DeserializeError::invalid_type(
                self.value, "a string",
            )),
        }
    }

    fn deserialize_unit<C: Converter<'de>>(self, converter: C) -> Result<C::Value, Self::Error> {
        match self.value {
            Value::Null => converter.convert_unit(),
            _ => Err(data_format::DeserializeError::invalid_type(
                self.value, "null",
            )),
        }
    }

    fn deserialize_list<C: Converter<'de>>(self, converter: C) -> Result<C::Value, Self::Error> {
        match self.value {
            Value::Array(list) => converter.convert_list(ValueListDeserializer::new(list)),
            _ => Err(data_format::DeserializeError::invalid_type(
                self.value, "a list",
            )),
        }
    }

    fn deserialize_map<C: Converter<'de>>(self, converter: C) -> Result<C::Value, Self::Error> {
        match self.value {
            Value::Object(map) => converter.convert_map(ValueMapDeserializer::new(map)),
            _ => Err(data_format::DeserializeError::invalid_type(
                self.value, "a map",
            )),
        }
    }

    fn deserialize_option<C: Converter<'de>>(self, converter: C) -> Result<C::Value, Self::Error> {
        match self.value {
            Value::Null => converter.convert_unit(),
            _ => converter.convert_some(self),
        }
    }
}
