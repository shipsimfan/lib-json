use crate::Value;

pub enum Type {
    Null,
    Boolean,
    String,
    Number,
    Array,
    Object,
}

impl Type {
    pub fn from_value(value: Value) -> Self {
        match value {
            Value::Null => Type::Null,
            Value::Boolean(_) => Type::Boolean,
            Value::String(_) => Type::String,
            Value::Number(_) => Type::Number,
            Value::Array(_) => Type::Array,
            Value::Object(_) => Type::Object,
        }
    }
}

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Type::Null => "null",
                Type::Boolean => "a boolean",
                Type::String => "a string",
                Type::Number => "a number",
                Type::Array => "an array",
                Type::Object => "an object",
            }
        )
    }
}
