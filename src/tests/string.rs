#[test]
fn string_empty() {
    super::run_success_test(&"".to_owned(), "\"\"")
}

#[test]
fn string_simple() {
    super::run_success_test(&"testing".to_owned(), "\"testing\"")
}

#[test]
fn string_escapes() {
    super::run_success_test(
        &"testing\"\t\r\nsome more \\".to_owned(),
        "\"testing\\\"\\t\\r\\nsome more \\\\\"",
    )
}
