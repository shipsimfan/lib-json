use crate::Value;
use rustc_hash::FxHashMap;
use std::{borrow::Cow, path::PathBuf};

pub trait Serialize: Sized {
    fn serialize(self) -> Value {
        self.serialize_ref()
    }

    fn serialize_ref(&self) -> Value;
}

impl Serialize for () {
    fn serialize_ref(&self) -> Value {
        Value::Null
    }
}

impl Serialize for bool {
    fn serialize_ref(&self) -> Value {
        Value::Boolean(*self)
    }
}

impl Serialize for Cow<'static, str> {
    fn serialize_ref(&self) -> Value {
        Value::String(self.clone())
    }

    fn serialize(self) -> Value {
        Value::String(self)
    }
}

impl Serialize for String {
    fn serialize_ref(&self) -> Value {
        Value::String(Cow::Owned(self.clone()))
    }

    fn serialize(self) -> Value {
        Value::String(Cow::Owned(self))
    }
}

impl Serialize for &'static str {
    fn serialize_ref(&self) -> Value {
        Value::String(Cow::Borrowed(self))
    }
}

impl Serialize for PathBuf {
    fn serialize_ref(&self) -> Value {
        Value::String(self.to_string_lossy().to_string().into())
    }
}

impl Serialize for f64 {
    fn serialize_ref(&self) -> Value {
        Value::Number(*self)
    }
}

impl Serialize for usize {
    fn serialize_ref(&self) -> Value {
        Value::Number(*self as f64)
    }
}

impl Serialize for isize {
    fn serialize_ref(&self) -> Value {
        Value::Number(*self as f64)
    }
}

impl<T: Serialize> Serialize for Vec<T> {
    fn serialize_ref(&self) -> Value {
        Value::Array(self.iter().map(|item| item.serialize_ref()).collect())
    }

    fn serialize(self) -> Value {
        Value::Array(self.into_iter().map(|item| item.serialize()).collect())
    }
}

impl<T: Serialize> Serialize for FxHashMap<Cow<'static, str>, T> {
    fn serialize_ref(&self) -> Value {
        let mut object = FxHashMap::default();
        for (key, item) in self {
            object.insert(key.to_owned(), item.serialize_ref());
        }
        Value::Object(object)
    }

    fn serialize(self) -> Value {
        let mut object = FxHashMap::default();
        for (key, item) in self {
            object.insert(key, item.serialize());
        }
        Value::Object(object)
    }
}

impl Serialize for FxHashMap<Cow<'static, str>, Value> {
    fn serialize_ref(&self) -> Value {
        let mut object = FxHashMap::default();
        for (key, item) in self {
            object.insert(key.to_owned(), item.clone());
        }
        Value::Object(object)
    }

    fn serialize(self) -> Value {
        Value::Object(self)
    }
}
