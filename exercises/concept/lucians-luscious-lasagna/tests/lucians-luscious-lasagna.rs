#[test]
fn expected_minutes_in_oven() {
    assert_eq!(40, lucians_luscious_lasagna::expected_minutes_in_oven());
}

#[ignore]
#[test]
fn remaining_minutes_in_oven() {
    assert_eq!(15, lucians_luscious_lasagna::remaining_minutes_in_oven(25));
}

#[ignore]
#[test]
fn preparation_time_in_minutes_for_one_layer() {
    assert_eq!(2, lucians_luscious_lasagna::preparation_time_in_minutes(1));
}

#[ignore]
#[test]
fn preparation_time_in_minutes_for_multiple_layers() {
    assert_eq!(8, lucians_luscious_lasagna::preparation_time_in_minutes(4));
}

#[ignore]
#[test]
fn elapsed_time_in_minutes_for_one_layer() {
    assert_eq!(32, lucians_luscious_lasagna::elapsed_time_in_minutes(1, 30));
}

#[ignore]
#[test]
fn elapsed_time_in_minutes_for_multiple_layers() {
    assert_eq!(16, lucians_luscious_lasagna::elapsed_time_in_minutes(4, 8));
}
