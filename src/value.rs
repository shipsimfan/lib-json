use data_format::{
    deserialize::{HashMapConverter, VecConverter},
    Converter, Deserialize, Serialize,
};
use std::{borrow::Cow, collections::HashMap};

/// A JSON value, representing any type in JSON
#[derive(Debug, Clone, PartialEq)]
pub enum Value<'de> {
    #[allow(missing_docs)]
    Null,

    #[allow(missing_docs)]
    Boolean(bool),

    #[allow(missing_docs)]
    Number(f64),

    #[allow(missing_docs)]
    String(Cow<'de, str>),

    #[allow(missing_docs)]
    Array(Vec<Value<'de>>),

    #[allow(missing_docs)]
    Object(HashMap<Cow<'de, str>, Value<'de>>),
}

struct ValueConverter;

impl<'de> Serialize for Value<'de> {
    fn serialize<S: data_format::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Value::Null => ().serialize(serializer),
            Value::Boolean(value) => value.serialize(serializer),
            Value::Number(value) => value.serialize(serializer),
            Value::String(value) => value.serialize(serializer),
            Value::Array(value) => value.serialize(serializer),
            Value::Object(value) => value.serialize(serializer),
        }
    }
}

impl<'de> Deserialize<'de> for Value<'de> {
    fn deserialize<D: data_format::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_any(ValueConverter)
    }
}

impl<'de> Converter<'de> for ValueConverter {
    type Value = Value<'de>;

    fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str("a JSON value")
    }

    fn convert_unit<E: data_format::DeserializeError<'de>>(self) -> Result<Self::Value, E> {
        Ok(Value::Null)
    }

    fn convert_bool<E: data_format::DeserializeError<'de>>(
        self,
        value: bool,
    ) -> Result<Self::Value, E> {
        Ok(Value::Boolean(value))
    }

    fn convert_i128<E: data_format::DeserializeError<'de>>(
        self,
        value: i128,
    ) -> Result<Self::Value, E> {
        Ok(Value::Number(value as f64))
    }

    fn convert_u128<E: data_format::DeserializeError<'de>>(
        self,
        value: u128,
    ) -> Result<Self::Value, E> {
        Ok(Value::Number(value as f64))
    }

    fn convert_f64<E: data_format::DeserializeError<'de>>(
        self,
        value: f64,
    ) -> Result<Self::Value, E> {
        Ok(Value::Number(value))
    }

    fn convert_str_borrow<E: data_format::DeserializeError<'de>>(
        self,
        value: Cow<'de, str>,
    ) -> Result<Self::Value, E> {
        Ok(Value::String(value))
    }

    fn convert_string<E: data_format::DeserializeError<'de>>(
        self,
        value: String,
    ) -> Result<Self::Value, E> {
        Ok(Value::String(value.into()))
    }

    fn convert_str<E: data_format::DeserializeError<'de>>(
        self,
        value: &str,
    ) -> Result<Self::Value, E> {
        Ok(Value::String(value.to_owned().into()))
    }

    fn convert_list<L: data_format::ListDeserializer<'de>>(
        self,
        list: L,
    ) -> Result<Self::Value, L::Error> {
        Ok(Value::Array(VecConverter::new().convert_list(list)?))
    }

    fn convert_map<M: data_format::MapDeserializer<'de>>(
        self,
        map: M,
    ) -> Result<Self::Value, M::Error> {
        Ok(Value::Object(HashMapConverter::new().convert_map(map)?))
    }
}
