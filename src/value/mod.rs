#[cfg(feature = "no-std")]
use alloc::{borrow::Cow, collections::BTreeMap, vec::Vec};
#[cfg(not(feature = "no-std"))]
use std::{borrow::Cow, collections::BTreeMap};

mod deserialize;
mod into;
mod serialize;

/// A JSON value, representing any type in JSON
#[derive(Debug, Clone, PartialEq)]
pub enum Value<'de> {
    #[allow(missing_docs)]
    Null,

    #[allow(missing_docs)]
    Boolean(bool),

    #[allow(missing_docs)]
    Number(f64),

    #[allow(missing_docs)]
    String(Cow<'de, str>),

    #[allow(missing_docs)]
    Array(Vec<Value<'de>>),

    #[allow(missing_docs)]
    Object(BTreeMap<Cow<'de, str>, Value<'de>>),
}
