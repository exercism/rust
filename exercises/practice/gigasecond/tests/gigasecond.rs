#[test]
fn date_only_specification_of_time() {
    let start = datetime(2011, 4, 25, 0, 0, 0);
    let actual = gigasecond::after(start);
    let expected = datetime(2043, 1, 1, 1, 46, 40);
    assert_eq!(actual, expected);
}

#[test]
#[ignore]
fn second_test_for_date_only_specification_of_time() {
    let start = datetime(1977, 6, 13, 0, 0, 0);
    let actual = gigasecond::after(start);
    let expected = datetime(2009, 2, 19, 1, 46, 40);
    assert_eq!(actual, expected);
}

#[test]
#[ignore]
fn third_test_for_date_only_specification_of_time() {
    let start = datetime(1959, 7, 19, 0, 0, 0);
    let actual = gigasecond::after(start);
    let expected = datetime(1991, 3, 27, 1, 46, 40);
    assert_eq!(actual, expected);
}

#[test]
#[ignore]
fn full_time_specified() {
    let start = datetime(2015, 1, 24, 22, 0, 0);
    let actual = gigasecond::after(start);
    let expected = datetime(2046, 10, 2, 23, 46, 40);
    assert_eq!(actual, expected);
}

#[test]
#[ignore]
fn full_time_with_day_roll_over() {
    let start = datetime(2015, 1, 24, 23, 59, 59);
    let actual = gigasecond::after(start);
    let expected = datetime(2046, 10, 3, 1, 46, 39);
    assert_eq!(actual, expected);
}

fn datetime(
    year: i32,
    month: u8,
    day: u8,
    hour: u8,
    minute: u8,
    second: u8,
) -> time::PrimitiveDateTime {
    use time::{Date, PrimitiveDateTime, Time};

    PrimitiveDateTime::new(
        Date::from_calendar_date(year, month.try_into().unwrap(), day).unwrap(),
        Time::from_hms(hour, minute, second).unwrap(),
    )
}
