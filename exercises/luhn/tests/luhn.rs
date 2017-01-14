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
