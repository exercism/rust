extern crate roman_numerals;

use roman_numerals::*;

#[test]
fn test_one() {
    assert_eq!("I", Roman::from(1));
}

#[test]
#[ignore]
fn test_two() {
    assert_eq!("II", Roman::from(2));
}

#[test]
#[ignore]
fn test_three() {
    assert_eq!("III", Roman::from(3));
}

#[test]
#[ignore]
fn test_four() {
    assert_eq!("IV", Roman::from(4));
}

#[test]
#[ignore]
fn test_five() {
    assert_eq!("V", Roman::from(5));
}

#[test]
#[ignore]
fn test_six() {
    assert_eq!("VI", Roman::from(6));
}

#[test]
#[ignore]
fn test_nine() {
    assert_eq!("IX", Roman::from(9));
}

#[test]
#[ignore]
fn test_twenty_seven() {
    assert_eq!("XXVII", Roman::from(27));
}

#[test]
#[ignore]
fn test_forty_eight() {
    assert_eq!("XLVIII", Roman::from(48));
}

#[test]
#[ignore]
fn test_fifty_nine() {
    assert_eq!("LIX", Roman::from(59));
}

#[test]
#[ignore]
fn test_ninety_three() {
    assert_eq!("XCIII", Roman::from(93));
}

#[test]
#[ignore]
fn test_141() {
    assert_eq!("CXLI", Roman::from(141));
}

#[test]
#[ignore]
fn test_163() {
    assert_eq!("CLXIII", Roman::from(163));
}

#[test]
#[ignore]
fn test_402() {
    assert_eq!("CDII", Roman::from(402));
}

#[test]
#[ignore]
fn test_575() {
    assert_eq!("DLXXV", Roman::from(575));
}

#[test]
#[ignore]
fn test_911() {
    assert_eq!("CMXI", Roman::from(911));
}

#[test]
#[ignore]
fn test_1024() {
    assert_eq!("MXXIV", Roman::from(1024));
}

#[test]
#[ignore]
fn test_3000() {
    assert_eq!("MMM", Roman::from(3000));
}
