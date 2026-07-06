use crate::Value;
use data_format::Serialize;

impl<'de> Serialize for Value<'de> {
    fn serialize<S: data_format::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Value::Null => ().serialize(serializer),
            Value::Boolean(boolean) => boolean.serialize(serializer),
            Value::Number(number) => number.serialize(serializer),
            Value::String(string) => string.serialize(serializer),
            Value::Array(array) => array.serialize(serializer),
            Value::Object(object) => object.serialize(serializer),
        }
    }
}
