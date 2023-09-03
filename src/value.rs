use rustc_hash::FxHashMap;
use std::borrow::Cow;

#[derive(Clone)]
pub enum Value {
    Null,
    Boolean(bool),
    String(Cow<'static, str>),
    Number(f64),
    Array(Box<[Value]>),
    Object(FxHashMap<String, Value>),
}

fn generate_string(string: &str, output: &mut Vec<u8>) {
    output.push(b'"');

    for byte in string.as_bytes() {
        if *byte <= 0x1F {
            match *byte {
                0x08 => output.extend(b"\\b"),
                0x0C => output.extend(b"\\f"),
                0x0A => output.extend(b"\\n"),
                0x0D => output.extend(b"\\r"),
                0x09 => output.extend(b"\\t"),
                _ => {
                    output.extend(b"\\u00");
                    output.extend(format!("{:X}", byte).as_bytes());
                }
            }
        } else if *byte == b'"' {
            output.extend(b"\\\"");
        } else if *byte == b'\\' {
            output.extend(b"\\\\");
        } else {
            output.push(*byte);
        }
    }

    output.push(b'"');
}

impl Value {
    pub fn is_null(&self) -> bool {
        match self {
            Value::Null => true,
            _ => false,
        }
    }

    pub fn is_boolean(&self) -> bool {
        match self {
            Value::Boolean(_) => true,
            _ => false,
        }
    }

    pub fn is_string(&self) -> bool {
        match self {
            Value::String(_) => true,
            _ => false,
        }
    }

    pub fn is_number(&self) -> bool {
        match self {
            Value::Number(_) => true,
            _ => false,
        }
    }

    pub fn is_array(&self) -> bool {
        match self {
            Value::Array(_) => true,
            _ => false,
        }
    }

    pub fn is_object(&self) -> bool {
        match self {
            Value::Object(_) => true,
            _ => false,
        }
    }

    pub fn to_boolean(self) -> Option<bool> {
        match self {
            Value::Boolean(value) => Some(value),
            _ => None,
        }
    }

    pub fn to_string(self) -> Option<String> {
        match self {
            Value::String(value) => Some(value.into()),
            _ => None,
        }
    }

    pub fn to_number(self) -> Option<f64> {
        match self {
            Value::Number(value) => Some(value),
            _ => None,
        }
    }

    pub fn to_array(self) -> Option<Box<[Value]>> {
        match self {
            Value::Array(value) => Some(value),
            _ => None,
        }
    }

    pub fn to_object(self) -> Option<FxHashMap<String, Value>> {
        match self {
            Value::Object(value) => Some(value),
            _ => None,
        }
    }

    pub fn generate(&self) -> Vec<u8> {
        let mut output = Vec::new();
        self.inner_generate(&mut output);
        output
    }

    fn inner_generate(&self, output: &mut Vec<u8>) {
        match self {
            Value::Null => output.extend(b"null"),
            Value::Boolean(value) => match value {
                true => output.extend(b"true"),
                false => output.extend(b"false"),
            },
            Value::String(string) => generate_string(string, output),
            Value::Number(number) => output.extend(number.to_string().as_bytes()),
            Value::Array(array) => {
                output.push(b'[');

                for i in 0..array.len() {
                    array[i].inner_generate(output);

                    if i < array.len() - 1 {
                        output.push(b',');
                    }
                }

                output.push(b']');
            }
            Value::Object(object) => {
                output.push(b'{');

                let mut i = object.len();
                for (key, value) in object {
                    generate_string(key, output);
                    output.push(b':');
                    value.inner_generate(output);

                    i -= 1;
                    if i > 0 {
                        output.push(b',');
                    }
                }

                output.push(b'}');
            }
        }
    }
}

impl From<()> for Value {
    fn from(_: ()) -> Self {
        Value::Null
    }
}

impl From<bool> for Value {
    fn from(value: bool) -> Self {
        Value::Boolean(value)
    }
}

impl From<Cow<'static, str>> for Value {
    fn from(value: Cow<'static, str>) -> Self {
        Value::String(value)
    }
}

impl From<&'static str> for Value {
    fn from(value: &'static str) -> Self {
        Value::String(value.into())
    }
}

impl From<String> for Value {
    fn from(value: String) -> Self {
        Value::String(value.into())
    }
}

impl From<u8> for Value {
    fn from(value: u8) -> Self {
        Value::Number(value as f64)
    }
}

impl From<u16> for Value {
    fn from(value: u16) -> Self {
        Value::Number(value as f64)
    }
}

impl From<u32> for Value {
    fn from(value: u32) -> Self {
        Value::Number(value as f64)
    }
}

impl From<u64> for Value {
    fn from(value: u64) -> Self {
        Value::Number(value as f64)
    }
}

impl From<u128> for Value {
    fn from(value: u128) -> Self {
        Value::Number(value as f64)
    }
}

impl From<i8> for Value {
    fn from(value: i8) -> Self {
        Value::Number(value as f64)
    }
}

impl From<i16> for Value {
    fn from(value: i16) -> Self {
        Value::Number(value as f64)
    }
}

impl From<i32> for Value {
    fn from(value: i32) -> Self {
        Value::Number(value as f64)
    }
}

impl From<i64> for Value {
    fn from(value: i64) -> Self {
        Value::Number(value as f64)
    }
}

impl From<i128> for Value {
    fn from(value: i128) -> Self {
        Value::Number(value as f64)
    }
}

impl From<f32> for Value {
    fn from(value: f32) -> Self {
        Value::Number(value as f64)
    }
}

impl From<f64> for Value {
    fn from(value: f64) -> Self {
        Value::Number(value)
    }
}

impl From<Vec<Value>> for Value {
    fn from(value: Vec<Value>) -> Self {
        Value::Array(value.into_boxed_slice())
    }
}

impl From<Box<[Value]>> for Value {
    fn from(value: Box<[Value]>) -> Self {
        Value::Array(value)
    }
}

impl From<FxHashMap<String, Value>> for Value {
    fn from(value: FxHashMap<String, Value>) -> Self {
        Value::Object(value)
    }
}
