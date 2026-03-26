#[cfg(feature = "no_std")]
use alloc::{borrow::ToOwned, collections::BTreeMap, string::String};
#[cfg(not(feature = "no_std"))]
use std::collections::BTreeMap;

#[test]
fn map_empty() {
    super::run_success_test(&BTreeMap::<String, ()>::default(), "{}")
}

#[test]
#[ignore = "HashMaps shuffle the order inconsistently, this test will fail randomly because of that"]
fn map_simple() {
    super::run_success_test(
        &BTreeMap::from([
            ("a".to_owned(), 1),
            ("b".to_owned(), 2),
            ("c".to_owned(), 3),
        ]),
        "{\"a\":1,\"b\":2,\"c\":3}",
    )
}
