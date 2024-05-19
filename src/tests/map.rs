use std::collections::HashMap;

#[test]
fn map_empty() {
    super::run_success_test(&HashMap::<String, ()>::default(), "{}")
}

#[test]
#[ignore = "HashMaps shuffle the order inconsistently, this test will fail randomly because of that"]
fn map_simple() {
    super::run_success_test(
        &HashMap::from([
            ("a".to_owned(), 1),
            ("b".to_owned(), 2),
            ("c".to_owned(), 3),
        ]),
        "{\"a\":1,\"b\":2,\"c\":3}",
    )
}
