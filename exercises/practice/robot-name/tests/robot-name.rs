use robot_name as robot;

fn assert_name_matches_pattern(n: &str) {
    assert!(n.len() == 5, "name is exactly 5 characters long");
    assert!(
        n[0..2].chars().all(|c| c.is_ascii_uppercase()),
        "name starts with 2 uppercase letters"
    );
    assert!(
        n[2..].chars().all(|c| c.is_ascii_digit()),
        "name ends with 3 numbers"
    );
}

#[test]
fn name_should_match_expected_pattern() {
    let r = robot::Robot::new();
    assert_name_matches_pattern(r.name());
}

#[test]
#[ignore]
fn different_robots_have_different_names() {
    let r1 = robot::Robot::new();
    let r2 = robot::Robot::new();
    assert_ne!(r1.name(), r2.name(), "Robot names should be different");
}

#[test]
#[ignore]
fn many_different_robots_have_different_names() {
    use std::collections::HashSet;

    // In 3,529 random robot names, there is ~99.99% chance of a name collision
    let vec: Vec<_> = (0..3529).map(|_| robot::Robot::new()).collect();
    let set: HashSet<_> = vec.iter().map(|robot| robot.name()).collect();

    let number_of_collisions = vec.len() - set.len();
    assert_eq!(number_of_collisions, 0);
}

#[test]
#[ignore]
fn new_name_should_match_expected_pattern() {
    let mut r = robot::Robot::new();
    assert_name_matches_pattern(r.name());
    r.reset_name();
    assert_name_matches_pattern(r.name());
}

#[test]
#[ignore]
fn new_name_is_different_from_old_name() {
    let mut r = robot::Robot::new();
    let n1 = r.name().to_string();
    r.reset_name();
    let n2 = r.name().to_string();
    assert_ne!(n1, n2, "Robot name should change when reset");
}
