use crate::{deserialize::value::ValueMapDeserializer, Value};
#[cfg(feature = "no-std")]
use alloc::{borrow::Cow, collections::btree_map::BTreeMap};
#[cfg(not(feature = "no-std"))]
use std::{borrow::Cow, collections::btree_map::BTreeMap};

impl<'de> ValueMapDeserializer<'de> {
    /// Creates a new [`ValueMapDeserializer`]
    pub fn new(map: BTreeMap<Cow<'de, str>, Value<'de>>) -> ValueMapDeserializer<'de> {
        ValueMapDeserializer {
            map: map.into_iter(),
            next_value: None,
        }
    }
}
