use allyourbase as ayb;

#[test]
fn single_bit_one_to_decimal() {
    let input_base = 2;
    let input_digits = &[1];
    let output_base = 10;
    let output_digits = vec![1];
    assert_eq!(
        ayb::convert(input_digits, input_base, output_base),
        Ok(output_digits)
    );
}

#[test]
#[ignore]
fn binary_to_single_decimal() {
    let input_base = 2;
    let input_digits = &[1, 0, 1];
    let output_base = 10;
    let output_digits = vec![5];
    assert_eq!(
        ayb::convert(input_digits, input_base, output_base),
        Ok(output_digits)
    );
}

#[test]
#[ignore]
fn single_decimal_to_binary() {
    let input_base = 10;
    let input_digits = &[5];
    let output_base = 2;
    let output_digits = vec![1, 0, 1];
    assert_eq!(
        ayb::convert(input_digits, input_base, output_base),
        Ok(output_digits)
    );
}

#[test]
#[ignore]
fn binary_to_multiple_decimal() {
    let input_base = 2;
    let input_digits = &[1, 0, 1, 0, 1, 0];
    let output_base = 10;
    let output_digits = vec![4, 2];
    assert_eq!(
        ayb::convert(input_digits, input_base, output_base),
        Ok(output_digits)
    );
}

#[test]
#[ignore]
fn decimal_to_binary() {
    let input_base = 10;
    let input_digits = &[4, 2];
    let output_base = 2;
    let output_digits = vec![1, 0, 1, 0, 1, 0];
    assert_eq!(
        ayb::convert(input_digits, input_base, output_base),
        Ok(output_digits)
    );
}

#[test]
#[ignore]
fn trinary_to_hexadecimal() {
    let input_base = 3;
    let input_digits = &[1, 1, 2, 0];
    let output_base = 16;
    let output_digits = vec![2, 10];
    assert_eq!(
        ayb::convert(input_digits, input_base, output_base),
        Ok(output_digits)
    );
}

#[test]
#[ignore]
fn hexadecimal_to_trinary() {
    let input_base = 16;
    let input_digits = &[2, 10];
    let output_base = 3;
    let output_digits = vec![1, 1, 2, 0];
    assert_eq!(
        ayb::convert(input_digits, input_base, output_base),
        Ok(output_digits)
    );
}

#[test]
#[ignore]
fn fifteen_bit_integer() {
    let input_base = 97;
    let input_digits = &[3, 46, 60];
    let output_base = 73;
    let output_digits = vec![6, 10, 45];
    assert_eq!(
        ayb::convert(input_digits, input_base, output_base),
        Ok(output_digits)
    );
}

#[test]
#[ignore]
fn empty_list() {
    let input_base = 2;
    let input_digits = &[];
    let output_base = 10;
    let output_digits = vec![0];
    assert_eq!(
        ayb::convert(input_digits, input_base, output_base),
        Ok(output_digits)
    );
}

#[test]
#[ignore]
fn single_zero() {
    let input_base = 10;
    let input_digits = &[0];
    let output_base = 2;
    let output_digits = vec![0];
    assert_eq!(
        ayb::convert(input_digits, input_base, output_base),
        Ok(output_digits)
    );
}

#[test]
#[ignore]
fn multiple_zeros() {
    let input_base = 10;
    let input_digits = &[0, 0, 0];
    let output_base = 2;
    let output_digits = vec![0];
    assert_eq!(
        ayb::convert(input_digits, input_base, output_base),
        Ok(output_digits)
    );
}

#[test]
#[ignore]
fn leading_zeros() {
    let input_base = 7;
    let input_digits = &[0, 6, 0];
    let output_base = 10;
    let output_digits = vec![4, 2];
    assert_eq!(
        ayb::convert(input_digits, input_base, output_base),
        Ok(output_digits)
    );
}

#[test]
#[ignore]
fn invalid_positive_digit() {
    let input_base = 2;
    let input_digits = &[1, 2, 1, 0, 1, 0];
    let output_base = 10;
    assert_eq!(
        ayb::convert(input_digits, input_base, output_base),
        Err(ayb::Error::InvalidDigit(2))
    );
}

#[test]
#[ignore]
fn input_base_is_one() {
    let input_base = 1;
    let input_digits = &[];
    let output_base = 10;
    assert_eq!(
        ayb::convert(input_digits, input_base, output_base),
        Err(ayb::Error::InvalidInputBase)
    );
}

#[test]
#[ignore]
fn output_base_is_one() {
    let input_base = 2;
    let input_digits = &[1, 0, 1, 0, 1, 0];
    let output_base = 1;
    assert_eq!(
        ayb::convert(input_digits, input_base, output_base),
        Err(ayb::Error::InvalidOutputBase)
    );
}

#[test]
#[ignore]
fn input_base_is_zero() {
    let input_base = 0;
    let input_digits = &[];
    let output_base = 10;
    assert_eq!(
        ayb::convert(input_digits, input_base, output_base),
        Err(ayb::Error::InvalidInputBase)
    );
}

#[test]
#[ignore]
fn output_base_is_zero() {
    let input_base = 10;
    let input_digits = &[7];
    let output_base = 0;
    assert_eq!(
        ayb::convert(input_digits, input_base, output_base),
        Err(ayb::Error::InvalidOutputBase)
    );
}
