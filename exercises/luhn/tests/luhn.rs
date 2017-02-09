extern crate luhn;

use luhn::*;

#[test]
fn single_digit_string_is_invalid() {
    assert!(!is_valid("1"));
}

#[test]
#[ignore]
fn single_zero_string_is_invalid() {
    assert!(!is_valid("0"));
}

#[test]
#[ignore]
fn simple_valid_sin() {
    assert!(is_valid(" 5 9 "));
}

#[test]
#[ignore]
fn valid_canadian_sin_is_valid() {
    assert!(is_valid("046 454 286"));
}

#[test]
#[ignore]
fn invalid_canadian_sin_is_invalid() {
    assert!(!is_valid("046 454 287"));
}

#[test]
#[ignore]
fn invalid_credit_card_is_invalid() {
    assert!(!is_valid("8273 1232 7352 0569"));
}

#[test]
#[ignore]
fn strings_that_contain_non_digits_are_invalid() {
    assert!(!is_valid("046a 454 286"));
}

#[test]
#[ignore]
fn punctuation_is_invalid() {
    assert!(!is_valid("055-444-285"));
}

#[test]
#[ignore]
fn symbols_are_invalid() {
    assert!(!is_valid("055£ 444$ 285"));
}

#[test]
#[ignore]
fn single_digit_with_space_is_invalid() {
    assert!(!is_valid(" 0"));
}

#[test]
#[ignore]
fn lots_of_zeros_are_valid() {
    assert!(is_valid(" 00000"));
}

#[test]
#[ignore]
fn another_valid_sin() {
    assert!(is_valid("055 444 285"));
}

#[test]
#[ignore]
fn nine_doubled_is_nine() {
    assert!(is_valid("091"));
}
