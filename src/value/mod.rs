mod array;
mod object;
mod string;

pub use array::Array;
pub use object::Object;
pub use string::String;

#[derive(Clone, PartialEq)]
pub enum Value<'a> {
    Object(Object<'a>),
    Array(Array<'a>),
    Number(f64),
    String(String<'a>),
    Boolean(bool),
    Null,
}

impl<'a> Value<'a> {
    pub fn borrow<'b>(&'b self) -> Value<'b> {
        match self {
            Value::Object(object) => Value::Object(object.borrow()),
            Value::Array(array) => Value::Array(array.borrow()),
            Value::Number(number) => Value::Number(*number),
            Value::String(string) => Value::String(string.borrow()),
            Value::Boolean(boolean) => Value::Boolean(*boolean),
            Value::Null => Value::Null,
        }
    }

    pub fn to_static(self) -> Value<'static> {
        match self {
            Value::Object(object) => Value::Object(object.to_static()),
            Value::Array(array) => Value::Array(array.to_static()),
            Value::Number(number) => Value::Number(number),
            Value::String(string) => Value::String(string.to_static()),
            Value::Boolean(boolean) => Value::Boolean(boolean),
            Value::Null => Value::Null,
        }
    }

    pub fn is_object(&self) -> bool {
        match self {
            Value::Object(_) => true,
            _ => false,
        }
    }

    pub fn is_array(&self) -> bool {
        match self {
            Value::Array(_) => true,
            _ => false,
        }
    }

    pub fn is_number(&self) -> bool {
        match self {
            Value::Number(_) => true,
            _ => false,
        }
    }

    pub fn is_string(&self) -> bool {
        match self {
            Value::String(_) => true,
            _ => false,
        }
    }

    pub fn is_boolean(&self) -> bool {
        match self {
            Value::Boolean(_) => true,
            _ => false,
        }
    }

    pub fn is_null(&self) -> bool {
        match self {
            Value::Null => true,
            _ => false,
        }
    }

    pub fn as_object(&self) -> Option<&Object<'a>> {
        match self {
            Value::Object(object) => Some(object),
            _ => None,
        }
    }

    pub fn as_array(&self) -> Option<&Array<'a>> {
        match self {
            Value::Array(array) => Some(array),
            _ => None,
        }
    }

    pub fn as_number(&self) -> Option<f64> {
        match self {
            Value::Number(number) => Some(*number),
            _ => None,
        }
    }

    pub fn as_string(&self) -> Option<&String<'a>> {
        match self {
            Value::String(string) => Some(string),
            _ => None,
        }
    }

    pub fn as_boolean(&self) -> Option<bool> {
        match self {
            Value::Boolean(boolean) => Some(*boolean),
            _ => None,
        }
    }

    pub fn as_null(self) -> Option<()> {
        match self {
            Value::Null => Some(()),
            _ => None,
        }
    }

    pub fn to_object(self) -> Option<Object<'a>> {
        match self {
            Value::Object(object) => Some(object),
            _ => None,
        }
    }

    pub fn to_array(self) -> Option<Array<'a>> {
        match self {
            Value::Array(array) => Some(array),
            _ => None,
        }
    }

    pub fn to_number(self) -> Option<f64> {
        match self {
            Value::Number(number) => Some(number),
            _ => None,
        }
    }

    pub fn to_string(self) -> Option<String<'a>> {
        match self {
            Value::String(string) => Some(string),
            _ => None,
        }
    }

    pub fn to_boolean(self) -> Option<bool> {
        match self {
            Value::Boolean(boolean) => Some(boolean),
            _ => None,
        }
    }

    pub fn to_null(self) -> Option<()> {
        match self {
            Value::Null => Some(()),
            _ => None,
        }
    }
}

impl<'a> std::fmt::Display for Value<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Object(object) => object.fmt(f),
            Value::Array(array) => array.fmt(f),
            Value::Number(number) => number.fmt(f),
            Value::String(string) => string.fmt(f),
            Value::Boolean(boolean) => boolean.fmt(f),
            Value::Null => "null".fmt(f),
        }
    }
}

impl<'a> std::fmt::Debug for Value<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self, f)
    }
}
