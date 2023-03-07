use crate::{Error, Type, Value};
use rustc_hash::FxHashMap;
use std::path::PathBuf;

pub trait Deserialize: Sized + 'static {
    fn deserialize(value: Value, key: Option<&str>) -> Result<Self, Error>;
}

impl Deserialize for () {
    fn deserialize(value: Value, key: Option<&str>) -> Result<Self, Error> {
        if value.is_null() {
            Ok(())
        } else {
            Err(Error::InvalidType(
                key.map(|key| key.to_string()),
                Type::Null,
                Type::from_value(value),
            ))
        }
    }
}

impl Deserialize for bool {
    fn deserialize(value: Value, key: Option<&str>) -> Result<Self, Error> {
        if value.is_boolean() {
            Ok(value.to_boolean().unwrap())
        } else {
            Err(Error::InvalidType(
                key.map(|key| key.to_string()),
                Type::Boolean,
                Type::from_value(value),
            ))
        }
    }
}

impl Deserialize for String {
    fn deserialize(value: Value, key: Option<&str>) -> Result<Self, Error> {
        if value.is_string() {
            Ok(value.to_string().unwrap())
        } else {
            Err(Error::InvalidType(
                key.map(|key| key.to_string()),
                Type::String,
                Type::from_value(value),
            ))
        }
    }
}

impl Deserialize for PathBuf {
    fn deserialize(value: Value, key: Option<&str>) -> Result<Self, Error> {
        String::deserialize(value, key).map(|string| PathBuf::from(string))
    }
}

impl Deserialize for f64 {
    fn deserialize(value: Value, key: Option<&str>) -> Result<Self, Error> {
        if value.is_number() {
            Ok(value.to_number().unwrap())
        } else {
            Err(Error::InvalidType(
                key.map(|key| key.to_string()),
                Type::Number,
                Type::from_value(value),
            ))
        }
    }
}

impl Deserialize for usize {
    fn deserialize(value: Value, key: Option<&str>) -> Result<Self, Error> {
        f64::deserialize(value, key).map(|float| float.trunc() as usize)
    }
}

impl<T: Deserialize> Deserialize for Vec<T> {
    fn deserialize(value: Value, key: Option<&str>) -> Result<Self, Error> {
        if !value.is_array() {
            return Err(Error::InvalidType(
                key.map(|key| key.to_string()),
                Type::Boolean,
                Type::from_value(value),
            ));
        }

        value
            .to_array()
            .unwrap()
            .into_vec()
            .into_iter()
            .enumerate()
            .map(|(i, value)| {
                T::deserialize(
                    value,
                    Some(&format!(
                        "{}[{}]",
                        match &key {
                            Some(key) => key,
                            None => "",
                        },
                        i
                    )),
                )
            })
            .collect()
    }
}

impl<T: Deserialize> Deserialize for FxHashMap<String, T> {
    fn deserialize(value: Value, key: Option<&str>) -> Result<Self, Error> {
        if value.is_object() {
            let original = value.to_object().unwrap().into_iter();
            let mut new = FxHashMap::default();

            for (map_key, value) in original {
                new.insert(map_key, T::deserialize(value, key)?);
            }

            Ok(new)
        } else {
            Err(Error::InvalidType(
                key.map(|key| key.to_string()),
                Type::String,
                Type::from_value(value),
            ))
        }
    }
}
