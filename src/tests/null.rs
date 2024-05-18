#[test]
fn null() {
    super::run_success_test((), "null")
}

#[test]
fn null_nul() {
    super::run_failed_test::<()>("nul")
}

#[test]
fn null_empty() {
    super::run_failed_test::<()>("")
}

#[test]
fn null_other() {
    super::run_failed_test::<()>("another")
}
