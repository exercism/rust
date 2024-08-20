use clock::*;

//
// Clock Creation
//

#[test]
fn on_the_hour() {
    assert_eq!(Clock::new(8, 0).to_string(), "08:00");
}

#[test]
#[ignore]
fn past_the_hour() {
    assert_eq!(Clock::new(11, 9).to_string(), "11:09");
}

#[test]
#[ignore]
fn midnight_is_zero_hours() {
    assert_eq!(Clock::new(24, 0).to_string(), "00:00");
}

#[test]
#[ignore]
fn hour_rolls_over() {
    assert_eq!(Clock::new(25, 0).to_string(), "01:00");
}

#[test]
#[ignore]
fn hour_rolls_over_continuously() {
    assert_eq!(Clock::new(100, 0).to_string(), "04:00");
}

#[test]
#[ignore]
fn sixty_minutes_is_next_hour() {
    assert_eq!(Clock::new(1, 60).to_string(), "02:00");
}

#[test]
#[ignore]
fn minutes_roll_over() {
    assert_eq!(Clock::new(0, 160).to_string(), "02:40");
}

#[test]
#[ignore]
fn minutes_roll_over_continuously() {
    assert_eq!(Clock::new(0, 1723).to_string(), "04:43");
}

#[test]
#[ignore]
fn hour_and_minutes_roll_over() {
    assert_eq!(Clock::new(25, 160).to_string(), "03:40");
}

#[test]
#[ignore]
fn hour_and_minutes_roll_over_continuously() {
    assert_eq!(Clock::new(201, 3001).to_string(), "11:01");
}

#[test]
#[ignore]
fn hour_and_minutes_roll_over_to_exactly_midnight() {
    assert_eq!(Clock::new(72, 8640).to_string(), "00:00");
}

#[test]
#[ignore]
fn negative_hour() {
    assert_eq!(Clock::new(-1, 15).to_string(), "23:15");
}

#[test]
#[ignore]
fn negative_hour_rolls_over() {
    assert_eq!(Clock::new(-25, 0).to_string(), "23:00");
}

#[test]
#[ignore]
fn negative_hour_rolls_over_continuously() {
    assert_eq!(Clock::new(-91, 0).to_string(), "05:00");
}

#[test]
#[ignore]
fn negative_minutes() {
    assert_eq!(Clock::new(1, -40).to_string(), "00:20");
}

#[test]
#[ignore]
fn negative_minutes_roll_over() {
    assert_eq!(Clock::new(1, -160).to_string(), "22:20");
}

#[test]
#[ignore]
fn negative_minutes_roll_over_continuously() {
    assert_eq!(Clock::new(1, -4820).to_string(), "16:40");
}

#[test]
#[ignore]
fn negative_sixty_minutes_is_previous_hour() {
    assert_eq!(Clock::new(2, -60).to_string(), "01:00");
}

#[test]
#[ignore]
fn negative_hour_and_minutes_both_roll_over() {
    assert_eq!(Clock::new(-25, -160).to_string(), "20:20");
}

#[test]
#[ignore]
fn negative_hour_and_minutes_both_roll_over_continuously() {
    assert_eq!(Clock::new(-121, -5810).to_string(), "22:10");
}

//
// Clock Math
//

#[test]
#[ignore]
fn add_minutes() {
    let clock = Clock::new(10, 0).add_minutes(3);
    assert_eq!(clock.to_string(), "10:03");
}

#[test]
#[ignore]
fn add_no_minutes() {
    let clock = Clock::new(6, 41).add_minutes(0);
    assert_eq!(clock.to_string(), "06:41");
}

#[test]
#[ignore]
fn add_to_next_hour() {
    let clock = Clock::new(0, 45).add_minutes(40);
    assert_eq!(clock.to_string(), "01:25");
}

#[test]
#[ignore]
fn add_more_than_one_hour() {
    let clock = Clock::new(10, 0).add_minutes(61);
    assert_eq!(clock.to_string(), "11:01");
}

#[test]
#[ignore]
fn add_more_than_two_hours_with_carry() {
    let clock = Clock::new(0, 45).add_minutes(160);
    assert_eq!(clock.to_string(), "03:25");
}

#[test]
#[ignore]
fn add_across_midnight() {
    let clock = Clock::new(23, 59).add_minutes(2);
    assert_eq!(clock.to_string(), "00:01");
}

#[test]
#[ignore]
fn add_more_than_one_day_1500_min_25_hrs() {
    let clock = Clock::new(5, 32).add_minutes(1500);
    assert_eq!(clock.to_string(), "06:32");
}

#[test]
#[ignore]
fn add_more_than_two_days() {
    let clock = Clock::new(1, 1).add_minutes(3500);
    assert_eq!(clock.to_string(), "11:21");
}

#[test]
#[ignore]
fn subtract_minutes() {
    let clock = Clock::new(10, 3).add_minutes(-3);
    assert_eq!(clock.to_string(), "10:00");
}

#[test]
#[ignore]
fn subtract_to_previous_hour() {
    let clock = Clock::new(10, 3).add_minutes(-30);
    assert_eq!(clock.to_string(), "09:33");
}

#[test]
#[ignore]
fn subtract_more_than_an_hour() {
    let clock = Clock::new(10, 3).add_minutes(-70);
    assert_eq!(clock.to_string(), "08:53");
}

#[test]
#[ignore]
fn subtract_across_midnight() {
    let clock = Clock::new(0, 3).add_minutes(-4);
    assert_eq!(clock.to_string(), "23:59");
}

#[test]
#[ignore]
fn subtract_more_than_two_hours() {
    let clock = Clock::new(0, 0).add_minutes(-160);
    assert_eq!(clock.to_string(), "21:20");
}

#[test]
#[ignore]
fn subtract_more_than_two_hours_with_borrow() {
    let clock = Clock::new(6, 15).add_minutes(-160);
    assert_eq!(clock.to_string(), "03:35");
}

#[test]
#[ignore]
fn subtract_more_than_one_day_1500_min_25_hrs() {
    let clock = Clock::new(5, 32).add_minutes(-1500);
    assert_eq!(clock.to_string(), "04:32");
}

#[test]
#[ignore]
fn subtract_more_than_two_days() {
    let clock = Clock::new(2, 20).add_minutes(-3000);
    assert_eq!(clock.to_string(), "00:20");
}

//
// Test Equality
//

#[test]
#[ignore]
fn clocks_with_same_time() {
    assert_eq!(Clock::new(15, 37), Clock::new(15, 37));
}

#[test]
#[ignore]
fn clocks_a_minute_apart() {
    assert_ne!(Clock::new(15, 36), Clock::new(15, 37));
}

#[test]
#[ignore]
fn clocks_an_hour_apart() {
    assert_ne!(Clock::new(14, 37), Clock::new(15, 37));
}

#[test]
#[ignore]
fn clocks_with_hour_overflow() {
    assert_eq!(Clock::new(10, 37), Clock::new(34, 37));
}

#[test]
#[ignore]
fn clocks_with_hour_overflow_by_several_days() {
    assert_eq!(Clock::new(3, 11), Clock::new(99, 11));
}

#[test]
#[ignore]
fn clocks_with_negative_hour() {
    assert_eq!(Clock::new(22, 40), Clock::new(-2, 40));
}

#[test]
#[ignore]
fn clocks_with_negative_hour_that_wraps() {
    assert_eq!(Clock::new(17, 3), Clock::new(-31, 3));
}

#[test]
#[ignore]
fn clocks_with_negative_hour_that_wraps_multiple_times() {
    assert_eq!(Clock::new(13, 49), Clock::new(-83, 49));
}

#[test]
#[ignore]
fn clocks_with_minute_overflow() {
    assert_eq!(Clock::new(0, 1), Clock::new(0, 1441));
}

#[test]
#[ignore]
fn clocks_with_minute_overflow_by_several_days() {
    assert_eq!(Clock::new(2, 2), Clock::new(2, 4322));
}

#[test]
#[ignore]
fn clocks_with_negative_minute() {
    assert_eq!(Clock::new(2, 40), Clock::new(3, -20));
}

#[test]
#[ignore]
fn clocks_with_negative_minute_that_wraps() {
    assert_eq!(Clock::new(4, 10), Clock::new(5, -1490));
}

#[test]
#[ignore]
fn clocks_with_negative_minute_that_wraps_multiple_times() {
    assert_eq!(Clock::new(6, 15), Clock::new(6, -4305));
}

#[test]
#[ignore]
fn clocks_with_negative_hours_and_minutes() {
    assert_eq!(Clock::new(7, 32), Clock::new(-12, -268));
}

#[test]
#[ignore]
fn clocks_with_negative_hours_and_minutes_that_wrap() {
    assert_eq!(Clock::new(18, 7), Clock::new(-54, -11513));
}

#[test]
#[ignore]
fn full_clock_and_zeroed_clock() {
    assert_eq!(Clock::new(24, 0), Clock::new(0, 0));
}
