extern crate phone_number as phone;

fn to_some_string(s: &str) -> Option<String> {
    Some(s.to_string())
}

#[test]
fn test_cleans_the_number() {
    assert_eq!(
        phone::number("(223) 456-7890"),
        to_some_string("2234567890")
    );
}

#[test]
fn test_cleans_numbers_with_dots() {
    assert_eq!(phone::number("223.456.7890"), to_some_string("2234567890"));
}

#[test]
fn test_cleans_numbers_with_multiple_spaces() {
    assert_eq!(
        phone::number("223 456   7890   "),
        to_some_string("2234567890")
    );
}

#[test]
fn test_invalid_when_9_digits() {
    assert_eq!(phone::number("123456789"), None);
}

#[test]
fn test_invalid_when_11_digits_does_not_start_with_a_1() {
    assert_eq!(phone::number("22234567890"), None);
}

#[test]
fn test_valid_when_11_digits_and_starting_with_1() {
    assert_eq!(phone::number("12234567890"), to_some_string("2234567890"));
}

#[test]
fn test_valid_when_11_digits_and_starting_with_1_even_with_punctuation() {
    assert_eq!(
        phone::number("+1 (223) 456-7890"),
        to_some_string("2234567890")
    );
}

#[test]
fn test_invalid_when_more_than_11_digits() {
    assert_eq!(phone::number("321234567890"), None);
}

#[test]
fn test_invalid_with_letters() {
    assert_eq!(phone::number("123-abc-7890"), None);
}

#[test]
fn test_invalid_with_punctuations() {
    assert_eq!(phone::number("123-@:!-7890"), None);
}

#[test]
fn test_invalid_if_area_code_does_not_start_with_2_9() {
    assert_eq!(phone::number("(123) 456-7890"), None);
}

#[test]
fn test_invalid_if_exchange_code_does_not_start_with_2_9() {
    assert_eq!(phone::number("(223) 056-7890"), None);
}
