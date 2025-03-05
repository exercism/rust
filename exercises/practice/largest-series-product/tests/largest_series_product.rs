use largest_series_product::*;

#[test]
fn return_is_a_result() {
    assert!(lsp("29", 2).is_ok());
}

#[test]
#[ignore]
fn finds_the_largest_product_if_span_equals_length() {
    assert_eq!(Ok(18), lsp("29", 2));
}

#[test]
#[ignore]
fn can_find_the_largest_product_of_2_with_numbers_in_order() {
    assert_eq!(Ok(72), lsp("0123456789", 2));
}

#[test]
#[ignore]
fn can_find_the_largest_product_of_2() {
    assert_eq!(Ok(48), lsp("576802143", 2));
}

#[test]
#[ignore]
fn can_find_the_largest_product_of_3_with_numbers_in_order() {
    assert_eq!(Ok(504), lsp("0123456789", 3));
}

#[test]
#[ignore]
fn can_find_the_largest_product_of_3() {
    assert_eq!(Ok(270), lsp("1027839564", 3));
}

#[test]
#[ignore]
fn can_find_the_largest_product_of_5_with_numbers_in_order() {
    assert_eq!(Ok(15_120), lsp("0123456789", 5));
}

#[test]
#[ignore]
fn can_get_the_largest_product_of_a_big_number() {
    assert_eq!(
        Ok(23_520),
        lsp("73167176531330624919225119674426574742355349194934", 6)
    );
}

#[test]
#[ignore]
fn reports_zero_if_the_only_digits_are_zero() {
    assert_eq!(Ok(0), lsp("0000", 2));
}

#[test]
#[ignore]
fn reports_zero_if_all_spans_include_zero() {
    assert_eq!(Ok(0), lsp("99099", 3));
}

#[test]
#[ignore]
fn rejects_span_longer_than_string_length() {
    assert_eq!(Err(Error::SpanTooLong), lsp("123", 4));
}

#[test]
#[ignore]
fn reports_1_for_empty_string_and_empty_product_0_span() {
    assert_eq!(Ok(1), lsp("", 0));
}

#[test]
#[ignore]
fn reports_1_for_nonempty_string_and_empty_product_0_span() {
    assert_eq!(Ok(1), lsp("123", 0));
}

#[test]
#[ignore]
fn rejects_empty_string_and_nonzero_span() {
    assert_eq!(Err(Error::SpanTooLong), lsp("", 1));
}

#[test]
#[ignore]
fn rejects_invalid_character_in_digits() {
    assert_eq!(Err(Error::InvalidDigit('a')), lsp("1234a5", 2));
}
