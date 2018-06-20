extern crate largest_series_product;

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
    assert_eq!(Ok(15120), lsp("0123456789", 5));
}

#[test]
#[ignore]
fn span_of_six_in_a_large_number() {
    assert_eq!(
        Ok(23520),
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

// There may be some confusion about whether this should be 1 or error.
// The reasoning for it being 1 is this:
// There is one 0-character string contained in the empty string.
// That's the empty string itself.
// The empty product is 1 (the identity for multiplication).
// Therefore LSP('', 0) is 1.
// It's NOT the case that LSP('', 0) takes max of an empty list.
// So there is no error.
// Compare against LSP('123', 4):
// There are zero 4-character strings in '123'.
// So LSP('123', 4) really DOES take the max of an empty list.
// So LSP('123', 4) errors and LSP('', 0) does NOT.
#[test]
#[ignore]
fn an_empty_string_and_no_span_returns_one() {
    assert_eq!(Ok(1), lsp("", 0));
}

#[test]
#[ignore]
fn a_non_empty_string_and_no_span_returns_one() {
    assert_eq!(Ok(1), lsp("123", 0));
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
