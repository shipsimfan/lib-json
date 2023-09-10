use crate::{String, Value};
use std::ops::Deref;

#[derive(Clone)]
pub struct Object<'a>(Vec<(String<'a>, Value<'a>)>);

impl<'a> Object<'a> {
    pub fn new() -> Self {
        Object(Vec::new())
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Object(Vec::with_capacity(capacity))
    }

    pub fn to_static(self) -> Object<'static> {
        Object(
            self.into_iter()
                .map(|(key, value)| (key.to_static(), value.to_static()))
                .collect(),
        )
    }
}

impl<'a, K: Into<String<'a>>, V: Into<Value<'a>>> From<Vec<(K, V)>> for Object<'a> {
    fn from(object: Vec<(K, V)>) -> Self {
        Object(
            object
                .into_iter()
                .map(|(key, value)| (key.into(), value.into()))
                .collect(),
        )
    }
}

impl<'a> Into<Vec<(String<'a>, Value<'a>)>> for Object<'a> {
    fn into(self) -> Vec<(String<'a>, Value<'a>)> {
        self.0
    }
}

impl<'a> PartialEq for Object<'a> {
    fn eq(&self, other: &Self) -> bool {
        if self.len() != other.len() {
            return false;
        }

        for i in 0..self.len() {
            if self[i].0 != other[i].0 || self[i].1 != other[i].1 {
                return false;
            }
        }

        true
    }
}

impl<'a> Deref for Object<'a> {
    type Target = Vec<(String<'a>, Value<'a>)>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> IntoIterator for Object<'a> {
    type Item = (String<'a>, Value<'a>);
    type IntoIter = std::vec::IntoIter<(String<'a>, Value<'a>)>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a, 'b> IntoIterator for &'b Object<'a> {
    type Item = &'b (String<'a>, Value<'a>);
    type IntoIter = std::slice::Iter<'b, (String<'a>, Value<'a>)>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a> std::fmt::Display for Object<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        '{'.fmt(f)?;

        let mut first = true;
        for (key, value) in self {
            if first {
                first = false;
            } else {
                ','.fmt(f)?;
            }
            write!(f, "{}:{}", key, value)?;
        }

        '}'.fmt(f)
    }
}

impl<'a> std::fmt::Debug for Object<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self, f)
    }
}
