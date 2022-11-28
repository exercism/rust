use armstrong_numbers::*;

#[test]
fn test_zero_is_an_armstrong_number() {
    assert!(is_armstrong_number(0))
}

#[test]
#[ignore]
fn test_single_digit_numbers_are_armstrong_numbers() {
    assert!(is_armstrong_number(5))
}

#[test]
#[ignore]
fn test_there_are_no_2_digit_armstrong_numbers() {
    assert!(!is_armstrong_number(10))
}

#[test]
#[ignore]
fn test_three_digit_armstrong_number() {
    assert!(is_armstrong_number(153))
}

#[test]
#[ignore]
fn test_three_digit_non_armstrong_number() {
    assert!(!is_armstrong_number(100))
}

#[test]
#[ignore]
fn test_four_digit_armstrong_number() {
    assert!(is_armstrong_number(9474))
}

#[test]
#[ignore]
fn test_four_digit_non_armstrong_number() {
    assert!(!is_armstrong_number(9475))
}

#[test]
#[ignore]
fn test_seven_digit_armstrong_number() {
    assert!(is_armstrong_number(9_926_315))
}

#[test]
#[ignore]
fn test_seven_digit_non_armstrong_number() {
    assert!(!is_armstrong_number(9_926_316))
}

#[test]
#[ignore]
fn test_nine_digit_armstrong_number() {
    assert!(is_armstrong_number(912_985_153));
}

#[test]
#[ignore]
fn test_nine_digit_non_armstrong_number() {
    assert!(!is_armstrong_number(999_999_999));
}

#[test]
#[ignore]
fn test_ten_digit_non_armstrong_number() {
    assert!(!is_armstrong_number(3_999_999_999));
}

#[test]
#[ignore]
fn test_ten_digit_fake_armstrong_number() {
    assert!(!is_armstrong_number(4_106_098_957));
}
