#[test]
fn bool_true() {
    super::run_success_test(true, "true")
}

#[test]
fn bool_false() {
    super::run_success_test(false, "false")
}

#[test]
fn bool_frue() {
    super::run_failed_test::<bool>("frue")
}

#[test]
fn bool_talse() {
    super::run_failed_test::<bool>("talse")
}

#[test]
fn bool_empty() {
    super::run_failed_test::<bool>("")
}

#[test]
fn bool_other() {
    super::run_failed_test::<bool>("another")
}
