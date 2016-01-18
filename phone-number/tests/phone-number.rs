extern crate phone_number as phone;

fn to_some_string(s: &str) -> Option<String> {
    Some(s.to_string())
}

#[test]
fn test_number_cleans() {
    assert_eq!(phone::number("(123) 456-7890"), to_some_string("1234567890"));
}

#[test]
#[ignore]
fn test_number_cleans_with_dots() {
    assert_eq!(phone::number("123.456.7890"), to_some_string("1234567890"));
}

#[test]
#[ignore]
fn test_valid_when_11_digits_and_first_is_1() {
    assert_eq!(phone::number("11234567890"), to_some_string("1234567890"));
}

#[test]
#[ignore]
fn test_invalid_when_11_digits() {
    assert_eq!(phone::number("21234567890"), None);
}

#[test]
#[ignore]
fn test_invalid_when_9_digits() {
    assert_eq!(phone::number("123456789"), None);
}

#[test]
#[ignore]
fn test_invalid_when_empty() {
    assert_eq!(phone::number(""), None);
}

#[test]
#[ignore]
fn test_invalid_when_no_digits_present() {
    assert_eq!(phone::number(" (-) "), None);
}

#[test]
#[ignore]
fn test_valid_with_leading_characters() {
    assert_eq!(phone::number("my number is 123 456 7890"), to_some_string("1234567890"));
}

#[test]
#[ignore]
fn test_valid_with_trailing_characters() {
    assert_eq!(phone::number("123 456 7890 - bob"), to_some_string("1234567890"));
}

#[test]
#[ignore]
fn test_area_code() {
    assert_eq!(phone::area_code("1234567890"), to_some_string("123"));
}

#[test]
#[ignore]
fn test_area_code_with_full_us_phone_number() {
    assert_eq!(phone::area_code("18234567890"), to_some_string("823"));
}

#[test]
#[ignore]
fn test_area_code_with_invalid() {
    assert_eq!(phone::area_code("1234161567890"), None);
}

#[test]
#[ignore]
fn test_pretty_print() {
    assert_eq!(phone::pretty_print("1234567890"), "(123) 456-7890");
}

#[test]
#[ignore]
fn test_pretty_print_with_full_us_phone_number() {
    assert_eq!(phone::pretty_print("11234567890"), "(123) 456-7890");
}

#[test]
#[ignore]
fn test_pretty_print_with_invalid() {
    assert_eq!(phone::pretty_print("1186234567890"), "invalid");
}
