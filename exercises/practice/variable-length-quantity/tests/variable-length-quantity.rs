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
    let input = &[8192];
    let output = vlq::to_bytes(input);
    let expected = vec![0xc0, 0x0];
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn largest_double_byte() {
    let input = &[16383];
    let output = vlq::to_bytes(input);
    let expected = vec![0xff, 0x7f];
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn smallest_triple_byte() {
    let input = &[16384];
    let output = vlq::to_bytes(input);
    let expected = vec![0x81, 0x80, 0x0];
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn arbitrary_triple_byte() {
    let input = &[1048576];
    let output = vlq::to_bytes(input);
    let expected = vec![0xc0, 0x80, 0x0];
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn largest_triple_byte() {
    let input = &[2097151];
    let output = vlq::to_bytes(input);
    let expected = vec![0xff, 0xff, 0x7f];
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn smallest_quadruple_byte() {
    let input = &[2097152];
    let output = vlq::to_bytes(input);
    let expected = vec![0x81, 0x80, 0x80, 0x0];
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn arbitrary_quadruple_byte() {
    let input = &[134217728];
    let output = vlq::to_bytes(input);
    let expected = vec![0xc0, 0x80, 0x80, 0x0];
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn largest_quadruple_byte() {
    let input = &[268435455];
    let output = vlq::to_bytes(input);
    let expected = vec![0xff, 0xff, 0xff, 0x7f];
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn smallest_quintuple_byte() {
    let input = &[268435456];
    let output = vlq::to_bytes(input);
    let expected = vec![0x81, 0x80, 0x80, 0x80, 0x0];
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn arbitrary_quintuple_byte() {
    let input = &[4278190080];
    let output = vlq::to_bytes(input);
    let expected = vec![0x8f, 0xf8, 0x80, 0x80, 0x0];
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn maximum_32_bit_integer_input() {
    let input = &[4294967295];
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
    let input = &[16384, 1193046];
    let output = vlq::to_bytes(input);
    let expected = vec![0x81, 0x80, 0x0, 0xc8, 0xe8, 0x56];
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn many_multi_byte_values() {
    let input = &[8192, 1193046, 268435455, 0, 16383, 16384];
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
    let expected = Ok(vec![8192]);
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn three_bytes() {
    let input = &[0xff, 0xff, 0x7f];
    let output = vlq::from_bytes(input);
    let expected = Ok(vec![2097151]);
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn four_bytes() {
    let input = &[0x81, 0x80, 0x80, 0x0];
    let output = vlq::from_bytes(input);
    let expected = Ok(vec![2097152]);
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn maximum_32_bit_integer() {
    let input = &[0x8f, 0xff, 0xff, 0xff, 0x7f];
    let output = vlq::from_bytes(input);
    let expected = Ok(vec![4294967295]);
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
    let expected = Ok(vec![8192, 1193046, 268435455, 0, 16383, 16384]);
    assert_eq!(output, expected);
}
