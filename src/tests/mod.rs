//! Tests and utilities for them

use data_format::{Deserialize, Serialize};
use std::fmt::Debug;

mod bool;
mod list;
mod null;
mod number;
mod string;

/// Runs a test which should always be successful
pub fn run_success_test<'a, T>(value: &T, expected: &'static str)
where
    T: Serialize + Deserialize<'a> + PartialEq + Debug + ?Sized,
{
    let json = crate::to_str(value).unwrap();
    assert_eq!(json, expected);

    let deserialized_value: T = crate::from_str(expected).unwrap();
    assert_eq!(&deserialized_value, value);
}

pub fn run_failed_test<'a, T: Deserialize<'a>>(json: &'static str) {
    assert!(crate::from_str::<T>(json).is_err())
}
