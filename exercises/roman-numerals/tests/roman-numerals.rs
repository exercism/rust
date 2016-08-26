extern crate roman_numerals;

use roman_numerals::*;

#[test]
fn test_one() {
    assert_eq!("I", roman::from(1).to_string());
}

#[test]
#[ignore]
fn test_two() {
    assert_eq!("II", roman::from(2).to_string());
}

#[test]
#[ignore]
fn test_three() {
    assert_eq!("III", roman::from(3).to_string());
}

#[test]
#[ignore]
fn test_four() {
    assert_eq!("IV", roman::from(4).to_string());
}

#[test]
#[ignore]
fn test_five() {
    assert_eq!("V", roman::from(5).to_string());
}

#[test]
#[ignore]
fn test_six() {
    assert_eq!("VI", roman::from(6).to_string());
}

#[test]
#[ignore]
fn test_nine() {
    assert_eq!("IX", roman::from(9).to_string());
}

#[test]
#[ignore]
fn test_twenty_seven() {
    assert_eq!("XXVII", roman::from(27).to_string());
}

#[test]
#[ignore]
fn test_forty_eight() {
    assert_eq!("XLVIII", roman::from(48).to_string());
}

#[test]
#[ignore]
fn test_fifty_nine() {
    assert_eq!("LIX", roman::from(59).to_string());
}

#[test]
#[ignore]
fn test_ninety_three() {
    assert_eq!("XCIII", roman::from(93).to_string());
}

#[test]
#[ignore]
fn test_141() {
    assert_eq!("CXLI", roman::from(141).to_string());
}

#[test]
#[ignore]
fn test_163() {
    assert_eq!("CLXIII", roman::from(163).to_string());
}

#[test]
#[ignore]
fn test_402() {
    assert_eq!("CDII", roman::from(402).to_string());
}

#[test]
#[ignore]
fn test_575() {
    assert_eq!("DLXXV", roman::from(575).to_string());
}

#[test]
#[ignore]
fn test_911() {
    assert_eq!("CMXI", roman::from(911).to_string());
}

#[test]
#[ignore]
fn test_1024() {
    assert_eq!("MXXIV", roman::from(1024).to_string());
}

#[test]
#[ignore]
fn test_3000() {
    assert_eq!("MMM", roman::from(3000).to_string());
}
