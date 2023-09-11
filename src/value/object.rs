use crate::{ObjectIter, PrettyPrintable, String, ToJSON, Value};
use std::{
    collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, LinkedList, VecDeque},
    ops::Deref,
};

use super::pretty::display_indent;

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

impl<'a> ToJSON for Object<'a> {
    fn object_iter(&self) -> Option<&dyn ObjectIter> {
        Some(self)
    }

    fn to_json<'b>(&'b self) -> Value<'b> {
        Value::Object(self.borrow())
    }
}

impl<'a> ObjectIter for Object<'a> {
    fn for_each(&self, f: &mut dyn FnMut(String, &dyn ToJSON) -> bool) {
        for (key, value) in self {
            if !f(key.borrow(), value) {
                break;
            }
        }
    }
}

impl<'a> PrettyPrintable for Object<'a> {
    fn pretty_print<O: crate::Output>(
        &self,
        output: &mut O,
        depth: usize,
        indent_size: usize,
    ) -> Result<(), O::Error> {
        if self.len() == 0 {
            return write!(output, "{{}}");
        }

        write!(output, "{{")?;

        let mut first = true;
        for (key, value) in self {
            if first {
                first = false;
            } else {
                write!(output, ",")?;
            }

            writeln!(output)?;
            display_indent(output, depth + 1, indent_size)?;
            write!(output, "{}:", key)?;
            value.pretty_print(output, depth + 1, indent_size)?;
        }

        writeln!(output)?;
        display_indent(output, depth, indent_size)?;
        write!(output, "}}")
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

impl<'a> From<Vec<(String<'a>, Value<'a>)>> for Object<'a> {
    fn from(object: Vec<(String<'a>, Value<'a>)>) -> Self {
        Object::Owned(object)
    }
}

impl<'a, K: Into<String<'a>>, V: Into<Value<'a>>> From<VecDeque<(K, V)>> for Object<'a> {
    fn from(object: VecDeque<(K, V)>) -> Self {
        Object::Owned(
            object
                .into_iter()
                .map(|(key, value)| (key.into(), value.into()))
                .collect(),
        )
    }
}

impl<'a, K: Into<String<'a>>, V: Into<Value<'a>>> From<LinkedList<(K, V)>> for Object<'a> {
    fn from(object: LinkedList<(K, V)>) -> Self {
        Object::Owned(
            object
                .into_iter()
                .map(|(key, value)| (key.into(), value.into()))
                .collect(),
        )
    }
}

impl<'a, K: Into<String<'a>>, V: Into<Value<'a>>, S> From<HashSet<(K, V), S>> for Object<'a> {
    fn from(object: HashSet<(K, V), S>) -> Self {
        Object::Owned(
            object
                .into_iter()
                .map(|(key, value)| (key.into(), value.into()))
                .collect(),
        )
    }
}

impl<'a, K: Into<String<'a>>, V: Into<Value<'a>>> From<BTreeSet<(K, V)>> for Object<'a> {
    fn from(object: BTreeSet<(K, V)>) -> Self {
        Object::Owned(
            object
                .into_iter()
                .map(|(key, value)| (key.into(), value.into()))
                .collect(),
        )
    }
}

impl<'a, K: Into<String<'a>>, V: Into<Value<'a>>> From<BinaryHeap<(K, V)>> for Object<'a> {
    fn from(object: BinaryHeap<(K, V)>) -> Self {
        Object::Owned(
            object
                .into_iter()
                .map(|(key, value)| (key.into(), value.into()))
                .collect(),
        )
    }
}

impl<'a, K: Into<String<'a>>, V: Into<Value<'a>>, S> From<HashMap<K, V, S>> for Object<'a> {
    fn from(object: HashMap<K, V, S>) -> Self {
        Object::Owned(
            object
                .into_iter()
                .map(|(key, value)| (key.into(), value.into()))
                .collect(),
        )
    }
}

impl<'a, K: Into<String<'a>>, V: Into<Value<'a>>> From<BTreeMap<K, V>> for Object<'a> {
    fn from(object: BTreeMap<K, V>) -> Self {
        Object::Owned(
            object
                .into_iter()
                .map(|(key, value)| (key.into(), value.into()))
                .collect(),
        )
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
