use crate::Value;
#[cfg(feature = "no-std")]
use alloc::{
    borrow::{Cow, ToOwned},
    string::String,
};
use data_format::{
    BTreeMapConverter, Converter, Deserialize, DeserializeError, Deserializer, ListDeserializer,
    MapDeserializer, VecConverter,
};
#[cfg(not(feature = "no-std"))]
use std::borrow::Cow;

impl<'de> Deserialize<'de> for Value<'de> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_any(ValueConverter)
    }
}

struct ValueConverter;

impl<'de> Converter<'de> for ValueConverter {
    type Value = Value<'de>;

    fn expecting(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.write_str("a JSON value")
    }

    fn convert_unit<E: DeserializeError<'de>>(self) -> Result<Self::Value, E> {
        Ok(Value::Null)
    }

    fn convert_bool<E: DeserializeError<'de>>(self, value: bool) -> Result<Self::Value, E> {
        Ok(Value::Boolean(value))
    }

    fn convert_i128<E: DeserializeError<'de>>(self, value: i128) -> Result<Self::Value, E> {
        Ok(Value::Number(value as f64))
    }

    fn convert_u128<E: DeserializeError<'de>>(self, value: u128) -> Result<Self::Value, E> {
        Ok(Value::Number(value as f64))
    }

    fn convert_f64<E: DeserializeError<'de>>(self, value: f64) -> Result<Self::Value, E> {
        Ok(Value::Number(value))
    }

    fn convert_str_borrow<E: DeserializeError<'de>>(
        self,
        value: Cow<'de, str>,
    ) -> Result<Self::Value, E> {
        Ok(Value::String(value))
    }

    fn convert_string<E: DeserializeError<'de>>(self, value: String) -> Result<Self::Value, E> {
        Ok(Value::String(value.into()))
    }

    fn convert_str<E: DeserializeError<'de>>(self, value: &str) -> Result<Self::Value, E> {
        Ok(Value::String(value.to_owned().into()))
    }

    fn convert_list<L: ListDeserializer<'de>>(self, list: L) -> Result<Self::Value, L::Error> {
        Ok(Value::Array(VecConverter::new().convert_list(list)?))
    }

    fn convert_map<M: MapDeserializer<'de>>(self, map: M) -> Result<Self::Value, M::Error> {
        Ok(Value::Object(BTreeMapConverter::new().convert_map(map)?))
    }
}

impl Default for ValueConverter {
    fn default() -> Self {
        ValueConverter
    }
}
