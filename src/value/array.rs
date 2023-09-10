use crate::{ArrayIter, ToJSON, Value};
use std::ops::Deref;

#[derive(Clone)]
pub enum Array<'a> {
    Owned(Vec<Value<'a>>),
    Borrowed(&'a [Value<'a>]),
}

impl<'a> Array<'a> {
    pub fn as_slice(&self) -> &[Value<'a>] {
        match self {
            Array::Owned(array) => array.as_slice(),
            Array::Borrowed(array) => array,
        }
    }

    pub fn is_borrowed(&self) -> bool {
        match self {
            Array::Owned(_) => false,
            Array::Borrowed(_) => true,
        }
    }

    pub fn is_owned(&self) -> bool {
        !self.is_borrowed()
    }

    pub fn borrow<'b>(&'b self) -> Array<'b> {
        Array::Borrowed(self.as_slice())
    }

    pub fn to_static(self) -> Array<'static> {
        Array::Owned(
            match self {
                Array::Owned(array) => array,
                Array::Borrowed(array) => array.to_owned(),
            }
            .into_iter()
            .map(|value| value.to_static())
            .collect(),
        )
    }
}

impl<'a> ToJSON for Array<'a> {
    fn array_iter(&self) -> Option<&dyn ArrayIter> {
        Some(self)
    }

    fn to_json<'b>(&'b self) -> Value<'b> {
        Value::Array(self.borrow())
    }
}

impl<'a> ArrayIter for Array<'a> {
    fn for_each(&self, f: &dyn Fn(&dyn ToJSON)) {
        self.as_slice().iter().for_each(|value| f(value))
    }
}

impl<'a> PartialEq for Array<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.as_slice().eq(other.as_slice())
    }
}

impl<'a> Deref for Array<'a> {
    type Target = [Value<'a>];

    fn deref(&self) -> &Self::Target {
        self.as_slice()
    }
}

impl<'a> From<Vec<Value<'a>>> for Array<'a> {
    fn from(array: Vec<Value<'a>>) -> Self {
        Array::Owned(array)
    }
}

impl<'a> From<&'a [Value<'a>]> for Array<'a> {
    fn from(array: &'a [Value<'a>]) -> Self {
        Array::Borrowed(array)
    }
}

impl<'a> FromIterator<Value<'a>> for Array<'a> {
    fn from_iter<T: IntoIterator<Item = Value<'a>>>(iter: T) -> Self {
        Array::Owned(iter.into_iter().collect())
    }
}

impl<'a> Into<Vec<Value<'a>>> for Array<'a> {
    fn into(self) -> Vec<Value<'a>> {
        match self {
            Array::Owned(array) => array,
            Array::Borrowed(array) => array.to_owned(),
        }
    }
}

impl<'a, 'b> IntoIterator for &'b Array<'a> {
    type Item = &'b Value<'a>;
    type IntoIter = std::slice::Iter<'b, Value<'a>>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a> std::fmt::Display for Array<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        '['.fmt(f)?;

        let mut first = true;
        for value in self {
            if first {
                first = false;
            } else {
                ",".fmt(f)?;
            }
            value.fmt(f)?;
        }

        ']'.fmt(f)
    }
}

impl<'a> std::fmt::Debug for Array<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self, f)
    }
}
