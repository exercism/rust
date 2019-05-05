extern crate clock;

use clock::Clock;

//
// Clock Creation
//

#[test]
fn test_on_the_hour() {
    assert_eq!("08:00", Clock::new(8, 0).to_string());;
}

#[test]
#[ignore]
fn test_past_the_hour() {
    assert_eq!("11:09", Clock::new(11, 9).to_string());;
}

#[test]
#[ignore]
fn test_midnight_is_zero_hours() {
    assert_eq!("00:00", Clock::new(24, 0).to_string());;
}

#[test]
#[ignore]
fn test_hour_rolls_over() {
    assert_eq!("01:00", Clock::new(25, 0).to_string());;
}

#[test]
#[ignore]
fn test_hour_rolls_over_continuously() {
    assert_eq!("04:00", Clock::new(100, 0).to_string());;
}

#[test]
#[ignore]
fn test_sixty_minutes_is_next_hour() {
    assert_eq!("02:00", Clock::new(1, 60).to_string());;
}

#[test]
#[ignore]
fn test_minutes_roll_over() {
    assert_eq!("02:40", Clock::new(0, 160).to_string());;
}

#[test]
#[ignore]
fn test_minutes_roll_over_continuously() {
    assert_eq!("04:43", Clock::new(0, 1723).to_string());;
}

#[test]
#[ignore]
fn test_hours_and_minutes_roll_over() {
    assert_eq!("03:40", Clock::new(25, 160).to_string());;
}

#[test]
#[ignore]
fn test_hours_and_minutes_roll_over_continuously() {
    assert_eq!("11:01", Clock::new(201, 3001).to_string());;
}

#[test]
#[ignore]
fn test_hours_and_minutes_roll_over_to_exactly_midnight() {
    assert_eq!("00:00", Clock::new(72, 8640).to_string());;
}

#[test]
#[ignore]
fn test_negative_hour() {
    assert_eq!("23:15", Clock::new(-1, 15).to_string());;
}

#[test]
#[ignore]
fn test_negative_hour_roll_over() {
    assert_eq!("23:00", Clock::new(-25, 00).to_string());;
}

#[test]
#[ignore]
fn test_negative_hour_roll_over_continuously() {
    assert_eq!("05:00", Clock::new(-91, 00).to_string());;
}

#[test]
#[ignore]
fn test_negative_minutes() {
    assert_eq!("00:20", Clock::new(1, -40).to_string());;
}

#[test]
#[ignore]
fn test_negative_minutes_roll_over() {
    assert_eq!("22:20", Clock::new(1, -160).to_string());;
}

#[test]
#[ignore]
fn test_negative_minutes_roll_over_continuously() {
    assert_eq!("16:40", Clock::new(1, -4820).to_string());;
}

#[test]
#[ignore]
fn test_negative_sixty_minutes_is_prev_hour() {
    assert_eq!("01:00", Clock::new(2, -60).to_string());;
}

#[test]
#[ignore]
fn test_negative_hour_and_minutes_both_roll_over() {
    assert_eq!("20:20", Clock::new(-25, -160).to_string());;
}

#[test]
#[ignore]
fn test_negative_hour_and_minutes_both_roll_over_continuously() {
    assert_eq!("22:10", Clock::new(-121, -5810).to_string());;
}

//
// Clock Math
//

#[test]
#[ignore]
fn test_add_minutes() {
    let clock = Clock::new(10, 0).add_minutes(3);
    assert_eq!("10:03", clock.to_string());
}

#[test]
#[ignore]
fn test_add_no_minutes() {
    let clock = Clock::new(6, 41).add_minutes(0);
    assert_eq!("06:41", clock.to_string());
}

#[test]
#[ignore]
fn test_add_to_next_hour() {
    let clock = Clock::new(0, 45).add_minutes(40);
    assert_eq!("01:25", clock.to_string());
}

#[test]
#[ignore]
fn test_add_more_than_one_hour() {
    let clock = Clock::new(10, 0).add_minutes(61);
    assert_eq!("11:01", clock.to_string());
}

#[test]
#[ignore]
fn test_add_more_than_two_hours_with_carry() {
    let clock = Clock::new(0, 45).add_minutes(160);
    assert_eq!("03:25", clock.to_string());
}

#[test]
#[ignore]
fn test_add_across_midnight() {
    let clock = Clock::new(23, 59).add_minutes(2);
    assert_eq!("00:01", clock.to_string());
}

#[test]
#[ignore]
fn test_add_more_than_one_day() {
    let clock = Clock::new(5, 32).add_minutes(1500);
    assert_eq!("06:32", clock.to_string());
}

#[test]
#[ignore]
fn test_add_more_than_two_days() {
    let clock = Clock::new(1, 1).add_minutes(3500);
    assert_eq!("11:21", clock.to_string());
}

#[test]
#[ignore]
fn test_subtract_minutes() {
    let clock = Clock::new(10, 3).add_minutes(-3);
    assert_eq!("10:00", clock.to_string());
}

#[test]
#[ignore]
fn test_subtract_to_previous_hour() {
    let clock = Clock::new(10, 3).add_minutes(-30);
    assert_eq!("09:33", clock.to_string());
}

#[test]
#[ignore]
fn test_subtract_more_than_an_hour() {
    let clock = Clock::new(10, 3).add_minutes(-70);
    assert_eq!("08:53", clock.to_string());
}

#[test]
#[ignore]
fn test_subtract_across_midnight() {
    let clock = Clock::new(0, 3).add_minutes(-4);
    assert_eq!("23:59", clock.to_string());
}

#[test]
#[ignore]
fn test_subtract_more_than_two_hours() {
    let clock = Clock::new(0, 0).add_minutes(-160);
    assert_eq!("21:20", clock.to_string());
}

#[test]
#[ignore]
fn test_subtract_more_than_two_hours_with_borrow() {
    let clock = Clock::new(6, 15).add_minutes(-160);
    assert_eq!("03:35", clock.to_string());
}

#[test]
#[ignore]
fn test_subtract_more_than_one_day() {
    let clock = Clock::new(5, 32).add_minutes(-1500);
    assert_eq!("04:32", clock.to_string());
}

#[test]
#[ignore]
fn test_subtract_mores_than_two_days() {
    let clock = Clock::new(2, 20).add_minutes(-3000);
    assert_eq!("00:20", clock.to_string());
}

//
// Test Equality
//

#[test]
#[ignore]
fn test_compare_clocks_for_equality() {
    assert_eq!(Clock::new(15, 37), Clock::new(15, 37));
}

#[test]
#[ignore]
fn test_compare_clocks_a_minute_apart() {
    assert_ne!(Clock::new(15, 36), Clock::new(15, 37));
}

#[test]
#[ignore]
fn test_compare_clocks_an_hour_apart() {
    assert_ne!(Clock::new(14, 37), Clock::new(15, 37));
}

#[test]
#[ignore]
fn test_compare_clocks_with_hour_overflow() {
    assert_eq!(Clock::new(10, 37), Clock::new(34, 37));
}

#[test]
#[ignore]
fn test_compare_clocks_with_hour_overflow_by_several_days() {
    assert_eq!(Clock::new(3, 11), Clock::new(99, 11));
}

#[test]
#[ignore]
fn test_compare_clocks_with_negative_hour() {
    assert_eq!(Clock::new(22, 40), Clock::new(-2, 40));
}

#[test]
#[ignore]
fn test_compare_clocks_with_negative_hour_that_wraps() {
    assert_eq!(Clock::new(17, 3), Clock::new(-31, 3));
}

#[test]
#[ignore]
fn test_compare_clocks_with_negative_hour_that_wraps_multiple_times() {
    assert_eq!(Clock::new(13, 49), Clock::new(-83, 49));
}

#[test]
#[ignore]
fn test_compare_clocks_with_minutes_overflow() {
    assert_eq!(Clock::new(0, 1), Clock::new(0, 1441));
}

#[test]
#[ignore]
fn test_compare_clocks_with_minutes_overflow_by_several_days() {
    assert_eq!(Clock::new(2, 2), Clock::new(2, 4322));
}

#[test]
#[ignore]
fn test_compare_clocks_with_negative_minute() {
    assert_eq!(Clock::new(2, 40), Clock::new(3, -20))
}

#[test]
#[ignore]
fn test_compare_clocks_with_negative_minute_that_wraps() {
    assert_eq!(Clock::new(4, 10), Clock::new(5, -1490))
}

#[test]
#[ignore]
fn test_compare_clocks_with_negative_minute_that_wraps_multiple() {
    assert_eq!(Clock::new(6, 15), Clock::new(6, -4305))
}

#[test]
#[ignore]
fn test_compare_clocks_with_negative_hours_and_minutes() {
    assert_eq!(Clock::new(7, 32), Clock::new(-12, -268))
}

#[test]
#[ignore]
fn test_compare_clocks_with_negative_hours_and_minutes_that_wrap() {
    assert_eq!(Clock::new(18, 7), Clock::new(-54, -11513))
}

#[test]
#[ignore]
fn test_compare_full_clock_and_zeroed_clock() {
    assert_eq!(Clock::new(24, 0), Clock::new(0, 0))
}
