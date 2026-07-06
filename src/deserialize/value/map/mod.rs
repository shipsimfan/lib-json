use crate::Value;
#[cfg(feature = "no-std")]
use alloc::{borrow::Cow, collections::btree_map::IntoIter};
#[cfg(not(feature = "no-std"))]
use std::{borrow::Cow, collections::btree_map::IntoIter};

mod map_deserializer;
mod new;

/// A deserializer for a JSON [`Value`] that is a map
pub(in crate::deserialize::value) struct ValueMapDeserializer<'de> {
    /// The map to be deserialized
    map: IntoIter<Cow<'de, str>, Value<'de>>,

    /// The next value to return from the map, if any
    next_value: Option<Value<'de>>,
}
