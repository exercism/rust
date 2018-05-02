extern crate trinary;
use trinary::*;

#[test]
fn trinary_1_is_decimal_1() {
    assert_eq!(to_decimal("1"), 1);
}

#[ignore]
#[test]
fn trinary_2_is_decimal_2() {
    assert_eq!(to_decimal("2"), 2);
}

#[ignore]
#[test]
fn trinary_10_is_decimal_3() {
    assert_eq!(to_decimal("10"), 3);
}

#[ignore]
#[test]
fn trinary_11_is_decimal_4() {
    assert_eq!(to_decimal("11"), 4);
}

#[ignore]
#[test]
fn trinary_100_is_decimal_9() {
    assert_eq!(to_decimal("100"), 9);
}

#[ignore]
#[test]
fn trinary_112_is_decimal_14() {
    assert_eq!(to_decimal("112"), 14);
}

#[ignore]
#[test]
fn trinary_222_is_decimal_26() {
    assert_eq!(to_decimal("222"), 26);
}

#[ignore]
#[test]
fn trinary_1122000120_is_decimal_32091() {
    assert_eq!(to_decimal("1122000120"), 32091);
}

#[ignore]
#[test]
fn trinary_1234_is_decimal_0() {
    assert_eq!(to_decimal("1234"), 0);
}

#[ignore]
#[test]
fn invalid_word_as_input_returns() {
    assert_eq!(to_decimal("carrot"), 0);
}

#[ignore]
#[test]
fn invalid_numbers_with_letters_as_input_returns_0() {
    assert_eq!(to_decimal("0a1b2c"), 0);
}
