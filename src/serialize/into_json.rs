use crate::{String, Value};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, LinkedList, VecDeque};

pub trait ToJSON {
    fn object_iter(&self) -> Option<&dyn ObjectIter> {
        None
    }

    fn array_iter(&self) -> Option<&dyn ArrayIter> {
        None
    }

    fn to_json<'b>(&'b self) -> Value<'b>;
}

pub trait ToJSONString {
    fn to_json_string(&self) -> String;
}

pub trait ObjectIter {
    fn for_each(&self, f: &dyn Fn(String, &dyn ToJSON));
}

pub trait ArrayIter {
    fn for_each(&self, f: &dyn Fn(&dyn ToJSON));
}

impl ToJSON for u8 {
    fn to_json<'b>(&'b self) -> Value<'b> {
        Value::Number(*self as f64)
    }
}

impl ToJSON for u16 {
    fn to_json<'b>(&'b self) -> Value<'b> {
        Value::Number(*self as f64)
    }
}

impl ToJSON for u32 {
    fn to_json<'b>(&'b self) -> Value<'b> {
        Value::Number(*self as f64)
    }
}

impl ToJSON for u64 {
    fn to_json<'b>(&'b self) -> Value<'b> {
        Value::Number(*self as f64)
    }
}

impl ToJSON for u128 {
    fn to_json<'b>(&'b self) -> Value<'b> {
        Value::Number(*self as f64)
    }
}

impl ToJSON for usize {
    fn to_json<'b>(&'b self) -> Value<'b> {
        Value::Number(*self as f64)
    }
}

impl ToJSON for i8 {
    fn to_json<'b>(&'b self) -> Value<'b> {
        Value::Number(*self as f64)
    }
}

impl ToJSON for i16 {
    fn to_json<'b>(&'b self) -> Value<'b> {
        Value::Number(*self as f64)
    }
}

impl ToJSON for i32 {
    fn to_json<'b>(&'b self) -> Value<'b> {
        Value::Number(*self as f64)
    }
}

impl ToJSON for i64 {
    fn to_json<'b>(&'b self) -> Value<'b> {
        Value::Number(*self as f64)
    }
}

impl ToJSON for i128 {
    fn to_json<'b>(&'b self) -> Value<'b> {
        Value::Number(*self as f64)
    }
}

impl ToJSON for isize {
    fn to_json<'b>(&'b self) -> Value<'b> {
        Value::Number(*self as f64)
    }
}

impl ToJSON for f32 {
    fn to_json<'b>(&'b self) -> Value<'b> {
        Value::Number(*self as f64)
    }
}

impl ToJSON for f64 {
    fn to_json<'b>(&'b self) -> Value<'b> {
        Value::Number(*self)
    }
}

impl ToJSON for char {
    fn to_json<'b>(&'b self) -> Value<'b> {
        Value::String(self.to_json_string())
    }
}

impl ToJSONString for char {
    fn to_json_string(&self) -> String {
        (*self).to_string().into()
    }
}

impl ToJSON for bool {
    fn to_json<'b>(&'b self) -> Value<'b> {
        Value::Boolean(*self)
    }
}

impl ToJSON for () {
    fn to_json<'b>(&'b self) -> Value<'b> {
        Value::Null
    }
}

impl<T: ToJSON> ToJSON for &[T] {
    fn array_iter(&self) -> Option<&dyn ArrayIter> {
        Some(self)
    }

    fn to_json<'b>(&'b self) -> Value<'b> {
        Value::Array(self.iter().map(|value| value.to_json()).collect())
    }
}

impl<T: ToJSON> ArrayIter for &[T] {
    fn for_each(&self, f: &dyn Fn(&dyn ToJSON)) {
        self.iter().for_each(|value| f(value))
    }
}

impl<K: ToJSONString, V: ToJSON> ToJSON for &[(K, V)] {
    fn object_iter(&self) -> Option<&dyn ObjectIter> {
        Some(self)
    }

    fn to_json<'b>(&'b self) -> Value<'b> {
        Value::Object(
            self.iter()
                .map(|(key, value)| (key.to_json_string(), value.to_json()))
                .collect(),
        )
    }
}

impl<K: ToJSONString, V: ToJSON> ObjectIter for &[(K, V)] {
    fn for_each(&self, f: &dyn Fn(String, &dyn ToJSON)) {
        self.iter()
            .for_each(|(key, value)| f(key.to_json_string(), value))
    }
}

impl ToJSON for &str {
    fn to_json<'b>(&'b self) -> Value<'b> {
        Value::String(self.to_json_string())
    }
}

impl ToJSONString for &str {
    fn to_json_string(&self) -> String {
        String((*self).into())
    }
}

impl ToJSON for std::string::String {
    fn to_json<'b>(&'b self) -> Value<'b> {
        Value::String(self.to_json_string())
    }
}

impl ToJSONString for std::string::String {
    fn to_json_string(&self) -> String {
        String(self.into())
    }
}

impl<T: ToJSON> ToJSON for Vec<T> {
    fn array_iter(&self) -> Option<&dyn ArrayIter> {
        Some(self)
    }

    fn to_json<'b>(&'b self) -> Value<'b> {
        Value::Array(self.iter().map(|value| value.to_json()).collect())
    }
}

impl<T: ToJSON> ArrayIter for Vec<T> {
    fn for_each(&self, f: &dyn Fn(&dyn ToJSON)) {
        self.iter().for_each(|value| f(value))
    }
}

impl<T: ToJSON> ToJSON for Box<[T]> {
    fn array_iter(&self) -> Option<&dyn ArrayIter> {
        Some(self)
    }

    fn to_json<'b>(&'b self) -> Value<'b> {
        Value::Array(self.iter().map(|value| value.to_json()).collect())
    }
}

impl<T: ToJSON> ArrayIter for Box<[T]> {
    fn for_each(&self, f: &dyn Fn(&dyn ToJSON)) {
        self.iter().for_each(|value| f(value))
    }
}

impl<T: ToJSON> ToJSON for VecDeque<T> {
    fn array_iter(&self) -> Option<&dyn ArrayIter> {
        Some(self)
    }

    fn to_json<'b>(&'b self) -> Value<'b> {
        Value::Array(self.iter().map(|value| value.to_json()).collect())
    }
}

impl<T: ToJSON> ArrayIter for VecDeque<T> {
    fn for_each(&self, f: &dyn Fn(&dyn ToJSON)) {
        self.iter().for_each(|value| f(value))
    }
}

impl<T: ToJSON> ToJSON for LinkedList<T> {
    fn array_iter(&self) -> Option<&dyn ArrayIter> {
        Some(self)
    }

    fn to_json<'b>(&'b self) -> Value<'b> {
        Value::Array(self.iter().map(|value| value.to_json()).collect())
    }
}

impl<T: ToJSON> ArrayIter for LinkedList<T> {
    fn for_each(&self, f: &dyn Fn(&dyn ToJSON)) {
        self.iter().for_each(|value| f(value))
    }
}

impl<T: ToJSON> ToJSON for BTreeSet<T> {
    fn array_iter(&self) -> Option<&dyn ArrayIter> {
        Some(self)
    }

    fn to_json<'b>(&'b self) -> Value<'b> {
        Value::Array(self.iter().map(|value| value.to_json()).collect())
    }
}

impl<T: ToJSON> ArrayIter for BTreeSet<T> {
    fn for_each(&self, f: &dyn Fn(&dyn ToJSON)) {
        self.iter().for_each(|value| f(value))
    }
}

impl<T: ToJSON> ToJSON for BinaryHeap<T> {
    fn array_iter(&self) -> Option<&dyn ArrayIter> {
        Some(self)
    }

    fn to_json<'b>(&'b self) -> Value<'b> {
        Value::Array(self.iter().map(|value| value.to_json()).collect())
    }
}

impl<T: ToJSON> ArrayIter for BinaryHeap<T> {
    fn for_each(&self, f: &dyn Fn(&dyn ToJSON)) {
        self.iter().for_each(|value| f(value))
    }
}

impl<T: ToJSON, S> ToJSON for HashSet<T, S> {
    fn array_iter(&self) -> Option<&dyn ArrayIter> {
        Some(self)
    }

    fn to_json<'b>(&'b self) -> Value<'b> {
        Value::Array(self.iter().map(|value| value.to_json()).collect())
    }
}

impl<T: ToJSON, S> ArrayIter for HashSet<T, S> {
    fn for_each(&self, f: &dyn Fn(&dyn ToJSON)) {
        self.iter().for_each(|value| f(value))
    }
}

impl<K: ToJSONString, V: ToJSON> ToJSON for Vec<(K, V)> {
    fn object_iter(&self) -> Option<&dyn ObjectIter> {
        Some(self)
    }

    fn to_json<'b>(&'b self) -> Value<'b> {
        Value::Object(
            self.iter()
                .map(|(key, value)| (key.to_json_string(), value.to_json()))
                .collect(),
        )
    }
}

impl<K: ToJSONString, V: ToJSON> ObjectIter for Vec<(K, V)> {
    fn for_each(&self, f: &dyn Fn(String, &dyn ToJSON)) {
        self.iter()
            .for_each(|(key, value)| f(key.to_json_string(), value))
    }
}

impl<K: ToJSONString, V: ToJSON> ToJSON for VecDeque<(K, V)> {
    fn object_iter(&self) -> Option<&dyn ObjectIter> {
        Some(self)
    }

    fn to_json<'b>(&'b self) -> Value<'b> {
        Value::Object(
            self.iter()
                .map(|(key, value)| (key.to_json_string(), value.to_json()))
                .collect(),
        )
    }
}

impl<K: ToJSONString, V: ToJSON> ObjectIter for VecDeque<(K, V)> {
    fn for_each(&self, f: &dyn Fn(String, &dyn ToJSON)) {
        self.iter()
            .for_each(|(key, value)| f(key.to_json_string(), value))
    }
}

impl<K: ToJSONString, V: ToJSON> ToJSON for BTreeMap<K, V> {
    fn object_iter(&self) -> Option<&dyn ObjectIter> {
        Some(self)
    }

    fn to_json<'b>(&'b self) -> Value<'b> {
        Value::Object(
            self.iter()
                .map(|(key, value)| (key.to_json_string(), value.to_json()))
                .collect(),
        )
    }
}

impl<K: ToJSONString, V: ToJSON> ObjectIter for BTreeMap<K, V> {
    fn for_each(&self, f: &dyn Fn(String, &dyn ToJSON)) {
        self.iter()
            .for_each(|(key, value)| f(key.to_json_string(), value))
    }
}

impl<K: ToJSONString, V: ToJSON, S> ToJSON for HashMap<K, V, S> {
    fn object_iter(&self) -> Option<&dyn ObjectIter> {
        Some(self)
    }

    fn to_json<'b>(&'b self) -> Value<'b> {
        Value::Object(
            self.iter()
                .map(|(key, value)| (key.to_json_string(), value.to_json()))
                .collect(),
        )
    }
}

impl<K: ToJSONString, V: ToJSON, S> ObjectIter for HashMap<K, V, S> {
    fn for_each(&self, f: &dyn Fn(String, &dyn ToJSON)) {
        self.iter()
            .for_each(|(key, value)| f(key.to_json_string(), value))
    }
}

impl<T: ToJSON> ToJSON for Box<T> {
    fn to_json<'b>(&'b self) -> Value<'b> {
        self.as_ref().to_json()
    }
}
