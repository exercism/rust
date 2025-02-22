use largest_series_product::*;

#[test]
fn return_is_a_result() {
    assert!(lsp("29", 2).is_ok());
}

#[test]
#[ignore]
fn find_the_largest_product_when_span_equals_length() {
    assert_eq!(Ok(18), lsp("29", 2));
}

#[test]
#[ignore]
fn find_the_largest_product_of_two_with_numbers_in_order() {
    assert_eq!(Ok(72), lsp("0123456789", 2));
}

#[test]
#[ignore]
fn find_the_largest_product_of_two_with_numbers_not_in_order() {
    assert_eq!(Ok(48), lsp("576802143", 2));
}

#[test]
#[ignore]
fn find_the_largest_product_of_three_with_numbers_in_order() {
    assert_eq!(Ok(504), lsp("0123456789", 3));
}

#[test]
#[ignore]
fn find_the_largest_product_of_three_with_numbers_not_in_order() {
    assert_eq!(Ok(270), lsp("1027839564", 3));
}

#[test]
#[ignore]
fn find_the_largest_product_of_five_with_numbers_in_order() {
    assert_eq!(Ok(15_120), lsp("0123456789", 5));
}

#[test]
#[ignore]
fn span_of_six_in_a_large_number() {
    assert_eq!(
        Ok(23_520),
        lsp("73167176531330624919225119674426574742355349194934", 6)
    );
}

#[test]
#[ignore]
fn returns_zero_if_number_is_zeros() {
    assert_eq!(Ok(0), lsp("0000", 2));
}

#[test]
#[ignore]
fn returns_zero_if_all_products_are_zero() {
    assert_eq!(Ok(0), lsp("99099", 3));
}

#[test]
#[ignore]
fn a_span_is_longer_than_number_is_an_error() {
    assert_eq!(Err(Error::SpanTooLong), lsp("123", 4));
}

#[test]
#[ignore]
fn empty_string_and_non_zero_span_is_an_error() {
    assert_eq!(Err(Error::SpanTooLong), lsp("", 1));
}

#[test]
#[ignore]
fn a_string_with_non_digits_is_an_error() {
    assert_eq!(Err(Error::InvalidDigit('a')), lsp("1234a5", 2));
}
