use crate::Value;
use data_format::Unexpected;

impl<'de> Into<Unexpected> for Value<'de> {
    fn into(self) -> Unexpected {
        match self {
            Value::Null => Unexpected::Unit,
            Value::Boolean(boolean) => Unexpected::Bool(boolean),
            Value::Number(number) => Unexpected::Float(number),
            Value::String(string) => Unexpected::String(string.into_owned()),
            Value::Array(_) => Unexpected::List,
            Value::Object(_) => Unexpected::Map,
        }
    }
}
