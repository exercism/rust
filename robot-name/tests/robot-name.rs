extern crate robot;

/*
These are the expected signatures:

impl Robot {
    pub fn new() -> Robot { ... }
    pub fn name<'a>(&'a self) -> &'a str { ... }
    pub fn reset_name(&mut self) { ... }
}
*/

fn assert_name_matches_pattern(n: &str) {
    assert!(n[0..3].chars().all(|c| c >= 'A' && c <= 'Z'), "name starts with 3 letters");
    assert!(n[3..].chars().all(|c| c >= '0' && c <= '9'), "name ends with 3 numbers");
}

fn assert_name_is_persistent(r: &robot::Robot) {
    // The type system already proves this, but why not.
    let n1 = r.name();
    let n2 = r.name();
    let n3 = r.name();
    assert_eq!(n1, n2);
    assert_eq!(n2, n3);
}

#[test]
#[ignore]
fn test_name_should_match_expected_pattern() {
    let r = robot::Robot::new();
    assert_name_matches_pattern(r.name());
}

#[test]
#[ignore]
fn test_name_is_persistent() {
    assert_name_is_persistent(&robot::Robot::new());
}

#[test]
#[ignore]
fn test_different_robots_have_different_names() {
    let r1 = robot::Robot::new();
    let r2 = robot::Robot::new();
    assert!(r1.name() != r2.name(), "Robot names should be different");
}

#[test]
#[ignore]
fn test_new_name_should_match_expected_pattern() {
    let mut r = robot::Robot::new();
    assert_name_matches_pattern(r.name());
    r.reset_name();
    assert_name_matches_pattern(r.name());
}

#[test]
#[ignore]
fn test_new_name_is_persistent() {
    let mut r = robot::Robot::new();
    r.reset_name();
    assert_name_is_persistent(&r);
}

#[test]
#[ignore]
fn test_new_name_is_different_from_old_name() {
    let mut r = robot::Robot::new();
    let n1 = r.name().to_string();
    r.reset_name();
    let n2 = r.name().to_string();
    assert!(n1 != n2, "Robot name should change when reset");
}
