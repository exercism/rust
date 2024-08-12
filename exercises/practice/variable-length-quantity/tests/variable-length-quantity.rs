use variable_length_quantity as vlq;

#[test]
fn zero() {
    let input = &[0];
    let output = vlq::to_bytes(input);
    let expected = vec![0x0];
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn arbitrary_single_byte() {
    let input = &[64];
    let output = vlq::to_bytes(input);
    let expected = vec![0x40];
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn largest_single_byte() {
    let input = &[127];
    let output = vlq::to_bytes(input);
    let expected = vec![0x7f];
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn smallest_double_byte() {
    let input = &[128];
    let output = vlq::to_bytes(input);
    let expected = vec![0x81, 0x0];
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn arbitrary_double_byte() {
    let input = &[8_192];
    let output = vlq::to_bytes(input);
    let expected = vec![0xc0, 0x0];
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn largest_double_byte() {
    let input = &[16_383];
    let output = vlq::to_bytes(input);
    let expected = vec![0xff, 0x7f];
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn smallest_triple_byte() {
    let input = &[16_384];
    let output = vlq::to_bytes(input);
    let expected = vec![0x81, 0x80, 0x0];
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn arbitrary_triple_byte() {
    let input = &[1_048_576];
    let output = vlq::to_bytes(input);
    let expected = vec![0xc0, 0x80, 0x0];
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn largest_triple_byte() {
    let input = &[2_097_151];
    let output = vlq::to_bytes(input);
    let expected = vec![0xff, 0xff, 0x7f];
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn smallest_quadruple_byte() {
    let input = &[2_097_152];
    let output = vlq::to_bytes(input);
    let expected = vec![0x81, 0x80, 0x80, 0x0];
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn arbitrary_quadruple_byte() {
    let input = &[134_217_728];
    let output = vlq::to_bytes(input);
    let expected = vec![0xc0, 0x80, 0x80, 0x0];
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn largest_quadruple_byte() {
    let input = &[268_435_455];
    let output = vlq::to_bytes(input);
    let expected = vec![0xff, 0xff, 0xff, 0x7f];
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn smallest_quintuple_byte() {
    let input = &[268_435_456];
    let output = vlq::to_bytes(input);
    let expected = vec![0x81, 0x80, 0x80, 0x80, 0x0];
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn arbitrary_quintuple_byte() {
    let input = &[4_278_190_080];
    let output = vlq::to_bytes(input);
    let expected = vec![0x8f, 0xf8, 0x80, 0x80, 0x0];
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn maximum_32_bit_integer_input() {
    let input = &[4_294_967_295];
    let output = vlq::to_bytes(input);
    let expected = vec![0x8f, 0xff, 0xff, 0xff, 0x7f];
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn two_single_byte_values() {
    let input = &[64, 127];
    let output = vlq::to_bytes(input);
    let expected = vec![0x40, 0x7f];
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn two_multi_byte_values() {
    let input = &[16_384, 1_193_046];
    let output = vlq::to_bytes(input);
    let expected = vec![0x81, 0x80, 0x0, 0xc8, 0xe8, 0x56];
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn many_multi_byte_values() {
    let input = &[8_192, 1_193_046, 268_435_455, 0, 16_383, 16_384];
    let output = vlq::to_bytes(input);
    let expected = vec![
        0xc0, 0x0, 0xc8, 0xe8, 0x56, 0xff, 0xff, 0xff, 0x7f, 0x0, 0xff, 0x7f, 0x81, 0x80, 0x0,
    ];
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn one_byte() {
    let input = &[0x7f];
    let output = vlq::from_bytes(input);
    let expected = Ok(vec![127]);
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn two_bytes() {
    let input = &[0xc0, 0x0];
    let output = vlq::from_bytes(input);
    let expected = Ok(vec![8_192]);
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn three_bytes() {
    let input = &[0xff, 0xff, 0x7f];
    let output = vlq::from_bytes(input);
    let expected = Ok(vec![2_097_151]);
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn four_bytes() {
    let input = &[0x81, 0x80, 0x80, 0x0];
    let output = vlq::from_bytes(input);
    let expected = Ok(vec![2_097_152]);
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn maximum_32_bit_integer() {
    let input = &[0x8f, 0xff, 0xff, 0xff, 0x7f];
    let output = vlq::from_bytes(input);
    let expected = Ok(vec![4_294_967_295]);
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn incomplete_sequence_causes_error() {
    let input = &[0xff];
    let output = vlq::from_bytes(input);
    let expected = Err(vlq::Error::IncompleteNumber);
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn incomplete_sequence_causes_error_even_if_value_is_zero() {
    let input = &[0x80];
    let output = vlq::from_bytes(input);
    let expected = Err(vlq::Error::IncompleteNumber);
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn multiple_values() {
    let input = &[
        0xc0, 0x0, 0xc8, 0xe8, 0x56, 0xff, 0xff, 0xff, 0x7f, 0x0, 0xff, 0x7f, 0x81, 0x80, 0x0,
    ];
    let output = vlq::from_bytes(input);
    let expected = Ok(vec![8_192, 1_193_046, 268_435_455, 0, 16_383, 16_384]);
    assert_eq!(output, expected);
}
