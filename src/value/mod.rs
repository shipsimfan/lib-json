use crate::{ArrayIter, ObjectIter, ToJSON};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, LinkedList, VecDeque};

mod array;
mod object;
mod pretty;
mod string;

pub use array::Array;
pub use object::Object;
pub use pretty::{PrettyPrintable, PrettyPrinter};
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

impl<'a> ToJSON for Value<'a> {
    fn array_iter(&self) -> Option<&dyn ArrayIter> {
        self.as_array().map(|array| array.array_iter().unwrap())
    }

    fn object_iter(&self) -> Option<&dyn ObjectIter> {
        self.as_object().map(|object| object.object_iter().unwrap())
    }

    fn to_json<'b>(&'b self) -> Value<'b> {
        self.borrow()
    }
}

impl<'a> PrettyPrintable for Value<'a> {
    fn pretty_print<O: crate::Output>(
        &self,
        output: &mut O,
        depth: usize,
        indent_size: usize,
    ) -> Result<(), O::Error> {
        match self {
            Value::Object(object) => object.pretty_print(output, depth, indent_size),
            Value::Array(array) => array.pretty_print(output, depth, indent_size),
            Value::Number(number) => write!(output, "{}", number),
            Value::String(string) => write!(output, "{}", string),
            Value::Boolean(boolean) => write!(output, "{}", boolean),
            Value::Null => write!(output, "null"),
        }
    }
}

impl<'a> From<Object<'a>> for Value<'a> {
    fn from(object: Object<'a>) -> Self {
        Value::Object(object)
    }
}

impl<'a> From<Vec<(String<'a>, Value<'a>)>> for Value<'a> {
    fn from(object: Vec<(String<'a>, Value<'a>)>) -> Self {
        Value::Object(object.into())
    }
}

impl<'a, K: Into<String<'a>>, V: Into<Value<'a>>> From<VecDeque<(K, V)>> for Value<'a> {
    fn from(object: VecDeque<(K, V)>) -> Self {
        Value::Object(object.into())
    }
}

impl<'a, K: Into<String<'a>>, V: Into<Value<'a>>> From<LinkedList<(K, V)>> for Value<'a> {
    fn from(object: LinkedList<(K, V)>) -> Self {
        Value::Object(object.into())
    }
}

impl<'a, K: Into<String<'a>>, V: Into<Value<'a>>, S> From<HashSet<(K, V), S>> for Value<'a> {
    fn from(object: HashSet<(K, V), S>) -> Self {
        Value::Object(object.into())
    }
}

impl<'a, K: Into<String<'a>>, V: Into<Value<'a>>> From<BTreeSet<(K, V)>> for Value<'a> {
    fn from(object: BTreeSet<(K, V)>) -> Self {
        Value::Object(object.into())
    }
}

impl<'a, K: Into<String<'a>>, V: Into<Value<'a>>> From<BinaryHeap<(K, V)>> for Value<'a> {
    fn from(object: BinaryHeap<(K, V)>) -> Self {
        Value::Object(object.into())
    }
}

impl<'a, K: Into<String<'a>>, V: Into<Value<'a>>, S> From<HashMap<K, V, S>> for Value<'a> {
    fn from(object: HashMap<K, V, S>) -> Self {
        Value::Object(object.into())
    }
}

impl<'a, K: Into<String<'a>>, V: Into<Value<'a>>> From<BTreeMap<K, V>> for Value<'a> {
    fn from(object: BTreeMap<K, V>) -> Self {
        Value::Object(object.into())
    }
}

impl<'a> From<Array<'a>> for Value<'a> {
    fn from(array: Array<'a>) -> Self {
        Value::Array(array)
    }
}

impl<'a> From<Vec<Value<'a>>> for Value<'a> {
    fn from(array: Vec<Value<'a>>) -> Self {
        Value::Array(array.into())
    }
}

impl<'a> From<&'a [Value<'a>]> for Value<'a> {
    fn from(array: &'a [Value<'a>]) -> Self {
        Value::Array(array.into())
    }
}

impl<'a, T: Into<Value<'a>>> From<VecDeque<T>> for Value<'a> {
    fn from(array: VecDeque<T>) -> Self {
        Value::Array(array.into())
    }
}

impl<'a, T: Into<Value<'a>>> From<LinkedList<T>> for Value<'a> {
    fn from(array: LinkedList<T>) -> Self {
        Value::Array(array.into())
    }
}

impl<'a, T: Into<Value<'a>>> From<BTreeSet<T>> for Value<'a> {
    fn from(array: BTreeSet<T>) -> Self {
        Value::Array(array.into())
    }
}

impl<'a, T: Into<Value<'a>>, S> From<HashSet<T, S>> for Value<'a> {
    fn from(array: HashSet<T, S>) -> Self {
        Value::Array(array.into())
    }
}

impl<'a, T: Into<Value<'a>>> From<BinaryHeap<T>> for Value<'a> {
    fn from(array: BinaryHeap<T>) -> Self {
        Value::Array(array.into())
    }
}

impl<'a> From<f64> for Value<'a> {
    fn from(number: f64) -> Self {
        Value::Number(number)
    }
}

impl<'a> From<f32> for Value<'a> {
    fn from(number: f32) -> Self {
        Value::Number(number as f64)
    }
}

impl<'a> From<usize> for Value<'a> {
    fn from(number: usize) -> Self {
        Value::Number(number as f64)
    }
}

impl<'a> From<u8> for Value<'a> {
    fn from(number: u8) -> Self {
        Value::Number(number as f64)
    }
}

impl<'a> From<u16> for Value<'a> {
    fn from(number: u16) -> Self {
        Value::Number(number as f64)
    }
}

impl<'a> From<u32> for Value<'a> {
    fn from(number: u32) -> Self {
        Value::Number(number as f64)
    }
}

impl<'a> From<u64> for Value<'a> {
    fn from(number: u64) -> Self {
        Value::Number(number as f64)
    }
}

impl<'a> From<u128> for Value<'a> {
    fn from(number: u128) -> Self {
        Value::Number(number as f64)
    }
}

impl<'a> From<isize> for Value<'a> {
    fn from(number: isize) -> Self {
        Value::Number(number as f64)
    }
}

impl<'a> From<i8> for Value<'a> {
    fn from(number: i8) -> Self {
        Value::Number(number as f64)
    }
}

impl<'a> From<i16> for Value<'a> {
    fn from(number: i16) -> Self {
        Value::Number(number as f64)
    }
}

impl<'a> From<i32> for Value<'a> {
    fn from(number: i32) -> Self {
        Value::Number(number as f64)
    }
}

impl<'a> From<i64> for Value<'a> {
    fn from(number: i64) -> Self {
        Value::Number(number as f64)
    }
}

impl<'a> From<i128> for Value<'a> {
    fn from(number: i128) -> Self {
        Value::Number(number as f64)
    }
}

impl<'a> From<String<'a>> for Value<'a> {
    fn from(string: String<'a>) -> Self {
        Value::String(string)
    }
}

impl<'a> From<char> for Value<'a> {
    fn from(string: char) -> Self {
        Value::String(string.into())
    }
}

impl<'a> From<std::string::String> for Value<'a> {
    fn from(string: std::string::String) -> Self {
        Value::String(string.into())
    }
}

impl<'a> From<&'a str> for Value<'a> {
    fn from(string: &'a str) -> Self {
        Value::String(string.into())
    }
}

impl<'a> From<bool> for Value<'a> {
    fn from(boolean: bool) -> Self {
        Value::Boolean(boolean)
    }
}

impl<'a> From<()> for Value<'a> {
    fn from(_: ()) -> Self {
        Value::Null
    }
}

impl<'a> TryInto<Object<'a>> for Value<'a> {
    type Error = ();

    fn try_into(self) -> Result<Object<'a>, Self::Error> {
        self.to_object().ok_or(())
    }
}

impl<'a> TryInto<Vec<(String<'a>, Value<'a>)>> for Value<'a> {
    type Error = ();

    fn try_into(self) -> Result<Vec<(String<'a>, Value<'a>)>, Self::Error> {
        self.to_object().map(|object| object.into()).ok_or(())
    }
}

impl<'a> TryInto<Array<'a>> for Value<'a> {
    type Error = ();

    fn try_into(self) -> Result<Array<'a>, Self::Error> {
        self.to_array().ok_or(())
    }
}

impl<'a> TryInto<Vec<Value<'a>>> for Value<'a> {
    type Error = ();

    fn try_into(self) -> Result<Vec<Value<'a>>, Self::Error> {
        self.to_array().map(|array| array.into()).ok_or(())
    }
}

impl<'a> TryInto<f64> for Value<'a> {
    type Error = ();

    fn try_into(self) -> Result<f64, Self::Error> {
        self.to_number().ok_or(())
    }
}

impl<'a> TryInto<f32> for Value<'a> {
    type Error = ();

    fn try_into(self) -> Result<f32, Self::Error> {
        self.to_number().map(|number| number as f32).ok_or(())
    }
}

impl<'a> TryInto<usize> for Value<'a> {
    type Error = ();

    fn try_into(self) -> Result<usize, Self::Error> {
        self.to_number().map(|number| number as usize).ok_or(())
    }
}

impl<'a> TryInto<u8> for Value<'a> {
    type Error = ();

    fn try_into(self) -> Result<u8, Self::Error> {
        self.to_number().map(|number| number as u8).ok_or(())
    }
}

impl<'a> TryInto<u16> for Value<'a> {
    type Error = ();

    fn try_into(self) -> Result<u16, Self::Error> {
        self.to_number().map(|number| number as u16).ok_or(())
    }
}

impl<'a> TryInto<u32> for Value<'a> {
    type Error = ();

    fn try_into(self) -> Result<u32, Self::Error> {
        self.to_number().map(|number| number as u32).ok_or(())
    }
}

impl<'a> TryInto<u64> for Value<'a> {
    type Error = ();

    fn try_into(self) -> Result<u64, Self::Error> {
        self.to_number().map(|number| number as u64).ok_or(())
    }
}

impl<'a> TryInto<u128> for Value<'a> {
    type Error = ();

    fn try_into(self) -> Result<u128, Self::Error> {
        self.to_number().map(|number| number as u128).ok_or(())
    }
}

impl<'a> TryInto<isize> for Value<'a> {
    type Error = ();

    fn try_into(self) -> Result<isize, Self::Error> {
        self.to_number().map(|number| number as isize).ok_or(())
    }
}

impl<'a> TryInto<i8> for Value<'a> {
    type Error = ();

    fn try_into(self) -> Result<i8, Self::Error> {
        self.to_number().map(|number| number as i8).ok_or(())
    }
}

impl<'a> TryInto<i16> for Value<'a> {
    type Error = ();

    fn try_into(self) -> Result<i16, Self::Error> {
        self.to_number().map(|number| number as i16).ok_or(())
    }
}

impl<'a> TryInto<i32> for Value<'a> {
    type Error = ();

    fn try_into(self) -> Result<i32, Self::Error> {
        self.to_number().map(|number| number as i32).ok_or(())
    }
}

impl<'a> TryInto<i64> for Value<'a> {
    type Error = ();

    fn try_into(self) -> Result<i64, Self::Error> {
        self.to_number().map(|number| number as i64).ok_or(())
    }
}

impl<'a> TryInto<i128> for Value<'a> {
    type Error = ();

    fn try_into(self) -> Result<i128, Self::Error> {
        self.to_number().map(|number| number as i128).ok_or(())
    }
}

impl<'a> TryInto<String<'a>> for Value<'a> {
    type Error = ();

    fn try_into(self) -> Result<String<'a>, Self::Error> {
        self.to_string().ok_or(())
    }
}

impl<'a> TryInto<std::string::String> for Value<'a> {
    type Error = ();

    fn try_into(self) -> Result<std::string::String, Self::Error> {
        self.to_string().map(|string| string.into()).ok_or(())
    }
}

impl<'a> TryInto<bool> for Value<'a> {
    type Error = ();

    fn try_into(self) -> Result<bool, Self::Error> {
        self.to_boolean().ok_or(())
    }
}

impl<'a> TryInto<()> for Value<'a> {
    type Error = ();

    fn try_into(self) -> Result<(), Self::Error> {
        self.to_null().ok_or(())
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
