#![feature(test)]
extern crate test;
use test::Bencher;

fn main() {
    println!("Hello, world!");
}

pub fn is_leap_year(year: u64) -> bool {
    //exhausts after only two checks
    if year % 100 == 0 {
        year % 400 == 0
    } else {
        year % 4 == 0
    }
}

pub fn is_leap_year_one_line(year: u64) -> bool {
    (year % 4 == 0) && ((year % 100 != 0) || (year % 400 == 0))
}

pub fn is_leap_year_match(year: u64) -> bool {
    match (year % 4, year % 100, year % 400) {
        (_, _, 0) => true,
        (_, 0, _) => false,
        (0, _, _) => true,
        _ => false,
    }
}

use time::{Date, Duration, Month};

pub fn is_leap_year_time(year: u64) -> bool {
    (Date::from_calendar_date(year as i32, Month::February, 28).unwrap() + Duration::DAY).day()
        == 29
}

use chrono::prelude::*;

pub fn is_leap_year_chrono(year: u64) -> bool {
    (Utc.ymd(year as i32, 2, 28) + chrono::Duration::days(1)).day() == 29
}

pub fn is_leap_year_naive(year: u64) -> bool {
    (NaiveDate::from_ymd(year as i32, 2, 28) + chrono::Duration::days(1)).day() == 29
}

#[bench]
fn test_ternary(b: &mut Bencher) {
    b.iter(|| is_leap_year(2000));
}

#[bench]
fn test_one_line(b: &mut Bencher) {
    b.iter(|| is_leap_year_one_line(2000));
}

#[bench]
fn test_match(b: &mut Bencher) {
    b.iter(|| is_leap_year_match(2000));
}

#[bench]
fn test_time(b: &mut Bencher) {
    b.iter(|| is_leap_year_time(2000));
}

#[bench]
fn test_chrono(b: &mut Bencher) {
    b.iter(|| is_leap_year_chrono(2000));
}

#[bench]
fn test_naive(b: &mut Bencher) {
    b.iter(|| is_leap_year_naive(2000));
}
