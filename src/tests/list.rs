#[test]
fn list_empty() {
    super::run_success_test(&Vec::<()>::new(), "[]")
}

#[test]
fn list_single() {
    super::run_success_test(&vec![1], "[1]")
}

#[test]
fn list_multiple() {
    super::run_success_test(&vec![1, 2, 3], "[1,2,3]")
}

#[test]
fn list_multi_level() {
    super::run_success_test(&vec![vec![1, 2, 3], vec![4, 5, 6]], "[[1,2,3],[4,5,6]]")
}
