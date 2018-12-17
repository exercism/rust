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
fn simple_valid_sin_that_remains_valid_if_reversed() {
    assert!(is_valid("059"));
}

#[test]
#[ignore]
fn simple_valid_sin_that_becomes_invalid_if_reversed() {
    assert!(is_valid("59"));
}

#[test]
#[ignore]
fn valid_canadian_sin_is_valid() {
    assert!(is_valid("055 444 285"));
}

#[test]
#[ignore]
fn invalid_canadian_sin_is_invalid() {
    assert!(!is_valid("055 444 286"));
}

#[test]
#[ignore]
fn invalid_credit_card_is_invalid() {
    assert!(!is_valid("8273 1232 7352 0569"));
}

#[test]
#[ignore]
fn valid_number_with_an_even_number_of_digits() {
    assert!(is_valid("095 245 88"));
}

#[test]
#[ignore]
fn strings_that_contain_non_digits_are_invalid() {
    assert!(!is_valid("055a 444 285"));
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
fn more_than_a_single_zero_is_valid() {
    assert!(is_valid("0000 0"));
}

#[test]
#[ignore]
fn input_digit_9_is_correctly_converted_to_output_digit_9() {
    assert!(is_valid("091"));
}

#[test]
#[ignore]
fn non_digit_isnt_converted_to_digit_by_ascii_value() {
    assert!(!is_valid(":9"));
}
