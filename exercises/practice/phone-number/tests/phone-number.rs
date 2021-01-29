use phone_number as phone;

fn process_clean_case(number: &str, expected: Option<&str>) {
    assert_eq!(phone::number(number), expected.map(|x| x.to_string()));
}

#[test]
fn test_cleans_the_number() {
    process_clean_case("(223) 456-7890", Some("2234567890"));
}

#[test]
#[ignore]
fn test_cleans_numbers_with_dots() {
    process_clean_case("223.456.7890", Some("2234567890"));
}

#[test]
#[ignore]
fn test_cleans_numbers_with_multiple_spaces() {
    process_clean_case("223 456   7890   ", Some("2234567890"));
}

#[test]
#[ignore]
fn test_invalid_when_9_digits() {
    process_clean_case("123456789", None);
}

#[test]
#[ignore]
fn test_invalid_when_11_digits_does_not_start_with_a_1() {
    process_clean_case("22234567890", None);
}

#[test]
#[ignore]
fn test_valid_when_11_digits_and_starting_with_1() {
    process_clean_case("12234567890", Some("2234567890"));
}

#[test]
#[ignore]
fn test_valid_when_11_digits_and_starting_with_1_even_with_punctuation() {
    process_clean_case("+1 (223) 456-7890", Some("2234567890"));
}

#[test]
#[ignore]
fn test_invalid_when_more_than_11_digits() {
    process_clean_case("321234567890", None);
}

#[test]
#[ignore]
fn test_invalid_with_letters() {
    process_clean_case("123-abc-7890", None);
}

#[test]
#[ignore]
fn test_invalid_with_punctuations() {
    process_clean_case("123-@:!-7890", None);
}

#[test]
#[ignore]
fn test_invalid_if_area_code_starts_with_1_on_valid_11digit_number() {
    process_clean_case("1 (123) 456-7890", None);
}

#[test]
#[ignore]
fn test_invalid_if_area_code_starts_with_0_on_valid_11digit_number() {
    process_clean_case("1 (023) 456-7890", None);
}

#[test]
#[ignore]
fn test_invalid_if_area_code_starts_with_1() {
    process_clean_case("(123) 456-7890", None);
}

#[test]
#[ignore]
fn test_invalid_if_exchange_code_starts_with_1() {
    process_clean_case("(223) 156-7890", None);
}

#[test]
#[ignore]
fn test_invalid_if_exchange_code_starts_with_0() {
    process_clean_case("(223) 056-7890", None);
}

#[test]
#[ignore]
fn test_invalid_if_exchange_code_starts_with_1_on_valid_11digit_number() {
    process_clean_case("1 (223) 156-7890", None);
}

#[test]
#[ignore]
fn test_invalid_if_exchange_code_starts_with_0_on_valid_11digit_number() {
    process_clean_case("1 (223) 056-7890", None);
}

#[test]
#[ignore]
fn test_invalid_if_area_code_starts_with_0() {
    process_clean_case("(023) 456-7890", None);
}
