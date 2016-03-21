extern crate gigasecond;

/*
 * Students,
 *
 * Rust does not currently have a library for handling Time. To solve this exercise
 * you'll need to use the Chrono 'crate' (which is Rust's term for an external library).
 *
 * The first time you run `cargo test`, the Chrono crate will automatically be downloaded
 * and installed. More information on crates can be found at
 * https://doc.rust-lang.org/book/guessing-game.html#generating-a-secret-number
 *
 * In order to use the crate, your solution will need to start with the two following lines
*/
extern crate chrono;
use chrono::*;

#[test]
fn test_date() {
    let start_date = UTC.ymd(2011, 4, 25).and_hms(0,0,0);
    assert_eq!(gigasecond::after(start_date), UTC.ymd(2043, 1, 1).and_hms(1,46,40));
}

#[test]
#[ignore]
fn test_another_date() {
    let start_date = UTC.ymd(1977, 6, 13).and_hms(0,0,0);
    assert_eq!(gigasecond::after(start_date), UTC.ymd(2009, 2, 19).and_hms(1,46,40));
}

#[test]
#[ignore]
fn test_third_date() {
    let start_date = UTC.ymd(1959, 7, 19).and_hms(0,0,0);
    assert_eq!(gigasecond::after(start_date), UTC.ymd(1991, 3, 27).and_hms(1,46,40));
}

#[test]
#[ignore]
fn test_datetime() {
    let start_date = UTC.ymd(2015, 1, 24).and_hms(22,0,0);
    assert_eq!(gigasecond::after(start_date), UTC.ymd(2046, 10, 2).and_hms(23,46,40));
}

#[test]
#[ignore]
fn test_another_datetime() {
    let start_date = UTC.ymd(2015, 1, 24).and_hms(23,59,59);
    assert_eq!(gigasecond::after(start_date), UTC.ymd(2046, 10, 3).and_hms(1,46,39));
}
