use secret_handshake::*;

#[test]
fn wink_for_1() {
    assert_eq!(commands(1), Some(vec!["wink"]))
}

#[test]
#[ignore]
fn double_blink_for_10() {
    assert_eq!(commands(2), Some(vec!["double blink"]))
}

#[test]
#[ignore]
fn close_your_eyes_for_100() {
    assert_eq!(commands(4), Some(vec!["close your eyes"]))
}

#[test]
#[ignore]
fn jump_for_1000() {
    assert_eq!(commands(8), Some(vec!["jump"]))
}

#[test]
#[ignore]
fn combine_two_actions() {
    assert_eq!(commands(3), Some(vec!["wink", "double blink"]))
}

#[test]
#[ignore]
fn reverse_two_actions() {
    assert_eq!(commands(19), Some(vec!["double blink", "wink"]))
}

#[test]
#[ignore]
fn reversing_one_action_gives_the_same_action() {
    assert_eq!(commands(24), Some(vec!["jump"]))
}

#[test]
#[ignore]
fn reversing_no_actions_still_gives_no_actions() {
    assert_eq!(commands(16), None)
}

#[test]
#[ignore]
fn all_possible_actions() {
    assert_eq!(
        commands(15),
        Some(vec!["wink", "double blink", "close your eyes", "jump"])
    )
}

#[test]
#[ignore]
fn reverse_all_possible_actions() {
    assert_eq!(
        commands(31),
        Some(vec!["jump", "close your eyes", "double blink", "wink"])
    )
}

#[test]
#[ignore]
fn do_nothing_for_zero() {
    assert_eq!(commands(0), None)
}
