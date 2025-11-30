use robot_name as robot;
use std::sync::{Once, RwLock};

static INIT: Once = Once::new();
static mut PREDICTABLE_SEQ_TEST_LOCK: *const RwLock<()> = std::ptr::null();

/// All tests other than names_should_not_follow_predictable_sequence need to take a read lock,
/// so that they will not run concurrently with names_should_not_follow_predictable_sequence, who
/// takes the write lock
fn predictable_seq_test_lock() -> &'static RwLock<()> {
    INIT.call_once(|| {
        let boxed = Box::new(RwLock::new(()));
        unsafe {
            PREDICTABLE_SEQ_TEST_LOCK = Box::into_raw(boxed);
        }
    });

    unsafe { &*PREDICTABLE_SEQ_TEST_LOCK }
}

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
    let _guard = predictable_seq_test_lock().read();
    let r = robot::Robot::new();
    assert_name_matches_pattern(r.name());
}

#[test]
#[ignore]
fn different_robots_have_different_names() {
    let _guard = predictable_seq_test_lock().read();
    let r1 = robot::Robot::new();
    let r2 = robot::Robot::new();
    assert_ne!(r1.name(), r2.name(), "Robot names should be different");
}

#[test]
#[ignore]
fn many_different_robots_have_different_names() {
    let _guard = predictable_seq_test_lock().read();
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
    let _guard = predictable_seq_test_lock().read();
    let mut r = robot::Robot::new();
    assert_name_matches_pattern(r.name());
    r.reset_name();
    assert_name_matches_pattern(r.name());
}

#[test]
#[ignore]
fn new_name_is_different_from_old_name() {
    let _guard = predictable_seq_test_lock().read();
    let mut r = robot::Robot::new();
    let n1 = r.name().to_string();
    r.reset_name();
    let n2 = r.name().to_string();
    assert_ne!(n1, n2, "Robot name should change when reset");
}

fn name_to_num(name: &str) -> u32 {
    let mut chars = name.chars();
    ((chars.next().unwrap() as u32) - b'A' as u32) * 26 * 10 * 10 * 10
        + ((chars.next().unwrap() as u32) - b'A' as u32) * 10 * 10 * 10
        + ((chars.next().unwrap() as u32) - b'0' as u32) * 10 * 10
        + ((chars.next().unwrap() as u32) - b'0' as u32) * 10
        + ((chars.next().unwrap() as u32) - b'0' as u32)
}

#[test]
#[ignore]
fn names_should_not_follow_predictable_sequence() {
    // needs to run exclusively, otherwise other tests may mess up predictability detection
    let _guard = predictable_seq_test_lock().write();
    let nums = (0..10)
        .into_iter()
        .map(|_| name_to_num(robot::Robot::new().name()))
        .collect::<Vec<_>>();

    // just checking the easiest kind of predictable sequences here: the difference between
    // numerical representation of consecutive names is always the same
    let d = nums[1] - nums[0];
    assert!(
        !nums.windows(2).all(|w| w[1] - w[0] == d),
        "Name sequence is very predictable: The difference between the numerical representation of names is {d}"
    )
}
