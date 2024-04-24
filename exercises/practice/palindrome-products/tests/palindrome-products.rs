use palindrome_products::*;

#[test]
/// test `Palindrome::new` with valid input
fn palindrome_new_return_some() {
    for v in [1, 11, 121, 12321, 1234321, 123454321, 543212345] {
        assert_eq!(Palindrome::new(v).expect("is a palindrome").into_inner(), v);
    }
}

#[test]
#[ignore]
/// test `Palindrome::new` with invalid input
fn palindrome_new_return_none() {
    for v in [12, 2322, 23443, 1233211, 8932343] {
        assert_eq!(Palindrome::new(v), None);
    }
}

#[test]
#[ignore]
fn find_the_smallest_palindrome_from_single_digit_factors() {
    let output = palindrome_products(1, 9).map(|(min, _)| min.into_inner());
    let expected = Some(1);
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn find_the_largest_palindrome_from_single_digit_factors() {
    let output = palindrome_products(1, 9).map(|(_, max)| max.into_inner());
    let expected = Some(9);
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn find_the_smallest_palindrome_from_double_digit_factors() {
    let output = palindrome_products(10, 99).map(|(min, _)| min.into_inner());
    let expected = Some(121);
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn find_the_largest_palindrome_from_double_digit_factors() {
    let output = palindrome_products(10, 99).map(|(_, max)| max.into_inner());
    let expected = Some(9009);
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn find_the_smallest_palindrome_from_triple_digit_factors() {
    let output = palindrome_products(100, 999).map(|(min, _)| min.into_inner());
    let expected = Some(10201);
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn find_the_largest_palindrome_from_triple_digit_factors() {
    let output = palindrome_products(100, 999).map(|(_, max)| max.into_inner());
    let expected = Some(906609);
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn find_the_smallest_palindrome_from_four_digit_factors() {
    let output = palindrome_products(1000, 9999).map(|(min, _)| min.into_inner());
    let expected = Some(1002001);
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn find_the_largest_palindrome_from_four_digit_factors() {
    let output = palindrome_products(1000, 9999).map(|(_, max)| max.into_inner());
    let expected = Some(99000099);
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn empty_result_for_smallest_if_no_palindrome_in_the_range() {
    let output = palindrome_products(1002, 1003).map(|(min, _)| min.into_inner());
    let expected = None;
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn empty_result_for_largest_if_no_palindrome_in_the_range() {
    let output = palindrome_products(15, 15).map(|(_, max)| max.into_inner());
    let expected = None;
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn error_result_for_smallest_if_min_is_more_than_max() {
    let output = palindrome_products(10000, 1).map(|(min, _)| min.into_inner());
    let expected = None;
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn error_result_for_largest_if_min_is_more_than_max() {
    let output = palindrome_products(2, 1).map(|(_, max)| max.into_inner());
    let expected = None;
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn smallest_product_does_not_use_the_smallest_factor() {
    let output = palindrome_products(3215, 4000).map(|(min, _)| min.into_inner());
    let expected = Some(10988901);
    assert_eq!(output, expected);
}
