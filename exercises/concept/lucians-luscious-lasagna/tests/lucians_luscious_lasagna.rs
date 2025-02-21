use lucians_luscious_lasagna::{
    elapsed_time_in_minutes, expected_minutes_in_oven, preparation_time_in_minutes,
    remaining_minutes_in_oven,
};

#[test]
fn expected_minutes_in_oven_is_correct() {
    assert_eq!(40, expected_minutes_in_oven());
}

#[ignore]
#[test]
fn remaining_minutes_in_oven_after_fifteen_minutes() {
    assert_eq!(15, remaining_minutes_in_oven(25));
}

#[ignore]
#[test]
fn preparation_time_in_minutes_for_one_layer() {
    assert_eq!(2, preparation_time_in_minutes(1));
}

#[ignore]
#[test]
fn preparation_time_in_minutes_for_multiple_layers() {
    assert_eq!(8, preparation_time_in_minutes(4));
}

#[ignore]
#[test]
fn elapsed_time_in_minutes_for_one_layer() {
    assert_eq!(32, elapsed_time_in_minutes(1, 30));
}

#[ignore]
#[test]
fn elapsed_time_in_minutes_for_multiple_layers() {
    assert_eq!(16, elapsed_time_in_minutes(4, 8));
}
