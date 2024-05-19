#[test]
fn number_int_zero() {
    super::run_success_test(&0, "0")
}

#[test]
fn number_int_positive_one() {
    super::run_success_test(&1, "1")
}

#[test]
fn number_int_positive_multiple() {
    super::run_success_test(&34, "34")
}

#[test]
fn number_int_negative_one() {
    super::run_success_test(&-1, "-1")
}

#[test]
fn number_int_negative_multiple() {
    super::run_success_test(&-34, "-34")
}

#[test]
fn number_float_zero() {
    super::run_success_test(&0.0, "0")
}

#[test]
fn number_float_positive() {
    super::run_success_test(&1.23, "1.23")
}

#[test]
fn number_float_negative() {
    super::run_success_test(&-1.23, "-1.23")
}
