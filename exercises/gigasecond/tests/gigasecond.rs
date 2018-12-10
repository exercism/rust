use gigasecond;

use chrono::{TimeZone, Utc};

#[test]
fn test_date() {
    let start_date = Utc.ymd(2011, 4, 25).and_hms(0, 0, 0);

    assert_eq!(
        gigasecond::after(start_date),
        Utc.ymd(2043, 1, 1).and_hms(1, 46, 40)
    );
}

#[test]
#[ignore]
fn test_another_date() {
    let start_date = Utc.ymd(1977, 6, 13).and_hms(0, 0, 0);

    assert_eq!(
        gigasecond::after(start_date),
        Utc.ymd(2009, 2, 19).and_hms(1, 46, 40)
    );
}

#[test]
#[ignore]
fn test_third_date() {
    let start_date = Utc.ymd(1959, 7, 19).and_hms(0, 0, 0);

    assert_eq!(
        gigasecond::after(start_date),
        Utc.ymd(1991, 3, 27).and_hms(1, 46, 40)
    );
}

#[test]
#[ignore]
fn test_datetime() {
    let start_date = Utc.ymd(2015, 1, 24).and_hms(22, 0, 0);

    assert_eq!(
        gigasecond::after(start_date),
        Utc.ymd(2046, 10, 2).and_hms(23, 46, 40)
    );
}

#[test]
#[ignore]
fn test_another_datetime() {
    let start_date = Utc.ymd(2015, 1, 24).and_hms(23, 59, 59);

    assert_eq!(
        gigasecond::after(start_date),
        Utc.ymd(2046, 10, 3).and_hms(1, 46, 39)
    );
}
