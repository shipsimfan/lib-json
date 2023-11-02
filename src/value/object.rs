use crate::{String, Value};
use std::ops::Deref;

#[derive(Clone)]
pub enum Object<'a> {
    Owned(Vec<(String<'a>, Value<'a>)>),
    Borrowed(&'a [(String<'a>, Value<'a>)]),
}

impl<'a> Object<'a> {
    pub fn as_slice(&self) -> &[(String<'a>, Value<'a>)] {
        match self {
            Object::Owned(object) => object.as_slice(),
            Object::Borrowed(object) => object,
        }
    }

    pub fn is_borrowed(&self) -> bool {
        match self {
            Object::Owned(_) => false,
            Object::Borrowed(_) => true,
        }
    }

    pub fn is_owned(&self) -> bool {
        !self.is_borrowed()
    }

    pub fn borrow<'b>(&'b self) -> Object<'b> {
        Object::Borrowed(self.as_slice())
    }

    pub fn to_static(self) -> Object<'static> {
        Object::Owned(
            match self {
                Object::Owned(object) => object,
                Object::Borrowed(object) => object.to_owned(),
            }
            .into_iter()
            .map(|(key, value)| (key.to_static(), value.to_static()))
            .collect(),
        )
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
    type Target = [(String<'a>, Value<'a>)];

    fn deref(&self) -> &Self::Target {
        self.as_slice()
    }
}

impl<'a> From<&'a [(String<'a>, Value<'a>)]> for Object<'a> {
    fn from(object: &'a [(String<'a>, Value<'a>)]) -> Self {
        Object::Borrowed(object)
    }
}

impl<'a> FromIterator<(String<'a>, Value<'a>)> for Object<'a> {
    fn from_iter<T: IntoIterator<Item = (String<'a>, Value<'a>)>>(iter: T) -> Self {
        Object::Owned(iter.into_iter().collect())
    }
}

impl<'a> Into<Vec<(String<'a>, Value<'a>)>> for Object<'a> {
    fn into(self) -> Vec<(String<'a>, Value<'a>)> {
        match self {
            Object::Owned(object) => object,
            Object::Borrowed(object) => object.to_owned(),
        }
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
