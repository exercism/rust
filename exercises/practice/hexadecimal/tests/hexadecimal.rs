#[test]
fn test_hex_1_is_decimal_1() {
    assert_eq!(Some(1), hexadecimal::hex_to_int("1"));
}

#[test]
#[ignore]
fn test_hex_c_is_decimal_12() {
    assert_eq!(Some(12), hexadecimal::hex_to_int("c"));
}

#[test]
#[ignore]
fn test_hex_10_is_decimal_16() {
    assert_eq!(Some(16), hexadecimal::hex_to_int("10"));
}

#[test]
#[ignore]
fn test_hex_af_is_decimal_175() {
    assert_eq!(Some(175), hexadecimal::hex_to_int("af"));
}

#[test]
#[ignore]
fn test_hex_100_is_decimal_256() {
    assert_eq!(Some(256), hexadecimal::hex_to_int("100"));
}

#[test]
#[ignore]
fn test_hex_19ace_is_decimal_105166() {
    assert_eq!(Some(105_166), hexadecimal::hex_to_int("19ace"));
}

#[test]
#[ignore]
fn test_invalid_hex_is_none() {
    assert_eq!(None, hexadecimal::hex_to_int("carrot"));
}

#[test]
#[ignore]
fn test_black() {
    assert_eq!(Some(0), hexadecimal::hex_to_int("0000000"));
}

#[test]
#[ignore]
fn test_white() {
    assert_eq!(Some(16_777_215), hexadecimal::hex_to_int("ffffff"));
}

#[test]
#[ignore]
fn test_yellow() {
    assert_eq!(Some(16_776_960), hexadecimal::hex_to_int("ffff00"));
}
