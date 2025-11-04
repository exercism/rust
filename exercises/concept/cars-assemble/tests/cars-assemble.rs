#[test]
fn success_rate_for_speed_zero() {
    assert_eq!(0.0, cars_assemble::success_rate(0));
}

#[test]
#[ignore]
fn success_rate_for_speed_one() {
    assert_eq!(1.0, cars_assemble::success_rate(1));
}

#[test]
#[ignore]
fn success_rate_for_speed_four() {
    assert_eq!(1.0, cars_assemble::success_rate(4));
}

#[test]
#[ignore]
fn success_rate_for_speed_five() {
    assert_eq!(0.9, cars_assemble::success_rate(5));
}

#[test]
#[ignore]
fn success_rate_for_speed_nine() {
    assert_eq!(0.8, cars_assemble::success_rate(9));
}

#[test]
#[ignore]
fn success_rate_for_speed_ten() {
    assert_eq!(0.77, cars_assemble::success_rate(10));
}

#[test]
#[ignore]
fn production_rate_per_hour_for_speed_zero() {
    assert_eq!(0.0, cars_assemble::production_rate_per_hour(0));
}

#[test]
#[ignore]
fn production_rate_per_hour_for_speed_one() {
    assert_eq!(221.0, cars_assemble::production_rate_per_hour(1));
}

#[test]
#[ignore]
fn production_rate_per_hour_for_speed_four() {
    assert_eq!(884.0, cars_assemble::production_rate_per_hour(4));
}

#[test]
#[ignore]
fn production_rate_per_hour_for_speed_seven() {
    assert_eq!(1392.3, cars_assemble::production_rate_per_hour(7));
}

#[test]
#[ignore]
fn production_rate_per_hour_for_speed_nine() {
    assert_eq!(1591.2, cars_assemble::production_rate_per_hour(9));
}

#[test]
#[ignore]
fn production_rate_per_hour_for_speed_ten() {
    assert_eq!(1701.7, cars_assemble::production_rate_per_hour(10));
}

#[test]
#[ignore]
fn working_items_per_minute_for_speed_zero() {
    assert_eq!(0, cars_assemble::working_items_per_minute(0));
}

#[test]
#[ignore]
fn working_items_per_minute_for_speed_one() {
    assert_eq!(3, cars_assemble::working_items_per_minute(1));
}

#[test]
#[ignore]
fn working_items_per_minute_for_speed_five() {
    assert_eq!(16, cars_assemble::working_items_per_minute(5));
}

#[test]
#[ignore]
fn working_items_per_minute_for_speed_eight() {
    let my_int = 5;
    let my_float: f64 = my_int.into();
    assert_eq!(my_float, 5.0);
    assert_eq!(26, cars_assemble::working_items_per_minute(8));
}
#[test]
#[ignore]
fn working_items_per_minute_for_speed_nine() {
    assert_eq!(26, cars_assemble::working_items_per_minute(9));
}

#[test]
#[ignore]
fn working_items_per_minute_for_speed_ten() {
    assert_eq!(28, cars_assemble::working_items_per_minute(10));
}
