use palindrome_products::*;
use std::collections::HashSet;

#[test]
fn find_the_smallest_palindrome_from_single_digit_factors() {
    let output = palindrome_products(1, 9);
    assert!(output.is_some());

    let (pal, _) = output.unwrap();
    assert_eq!(pal.value(), 1);
    assert_eq!(pal.into_factors(), HashSet::from([(1, 1)]));
}

#[test]
#[ignore]
fn find_the_largest_palindrome_from_single_digit_factors() {
    let output = palindrome_products(1, 9);
    assert!(output.is_some());

    let (_, pal) = output.unwrap();
    assert_eq!(pal.value(), 9);
    assert_eq!(pal.into_factors(), HashSet::from([(1, 9), (3, 3)]));
}

#[test]
#[ignore]
fn find_the_smallest_palindrome_from_double_digit_factors() {
    let output = palindrome_products(10, 99);
    assert!(output.is_some());

    let (pal, _) = output.unwrap();
    assert_eq!(pal.value(), 121);
    assert_eq!(pal.into_factors(), HashSet::from([(11, 11)]));
}

#[test]
#[ignore]
fn find_the_largest_palindrome_from_double_digit_factors() {
    let output = palindrome_products(10, 99);
    assert!(output.is_some());

    let (_, pal) = output.unwrap();
    assert_eq!(pal.value(), 9009);
    assert_eq!(pal.into_factors(), HashSet::from([(91, 99)]));
}

#[test]
#[ignore]
fn find_the_smallest_palindrome_from_triple_digit_factors() {
    let output = palindrome_products(100, 999);
    assert!(output.is_some());

    let (pal, _) = output.unwrap();
    assert_eq!(pal.value(), 10201);
    assert_eq!(pal.into_factors(), HashSet::from([(101, 101)]));
}

#[test]
#[ignore]
fn find_the_largest_palindrome_from_triple_digit_factors() {
    let output = palindrome_products(100, 999);
    assert!(output.is_some());

    let (_, pal) = output.unwrap();
    assert_eq!(pal.value(), 906609);
    assert_eq!(pal.into_factors(), HashSet::from([(913, 993)]));
}

#[test]
#[ignore]
fn find_the_smallest_palindrome_from_four_digit_factors() {
    let output = palindrome_products(1000, 9999);
    assert!(output.is_some());

    let (pal, _) = output.unwrap();
    assert_eq!(pal.value(), 1002001);
    assert_eq!(pal.into_factors(), HashSet::from([(1001, 1001)]));
}

#[test]
#[ignore]
fn find_the_largest_palindrome_from_four_digit_factors() {
    let output = palindrome_products(1000, 9999);
    assert!(output.is_some());

    let (_, pal) = output.unwrap();
    assert_eq!(pal.value(), 99000099);
    assert_eq!(pal.into_factors(), HashSet::from([(9901, 9999)]));
}

#[test]
#[ignore]
fn empty_result_for_smallest_if_no_palindrome_in_the_range() {
    let output = palindrome_products(1002, 1003);
    assert!(output.is_none());
}

#[test]
#[ignore]
fn empty_result_for_largest_if_no_palindrome_in_the_range() {
    let output = palindrome_products(15, 15);
    assert!(output.is_none());
}

#[test]
#[ignore]
fn error_result_for_smallest_if_min_is_more_than_max() {
    let output = palindrome_products(10000, 1);
    assert!(output.is_none());
}

#[test]
#[ignore]
fn error_result_for_largest_if_min_is_more_than_max() {
    let output = palindrome_products(2, 1);
    assert!(output.is_none());
}

#[test]
#[ignore]
fn smallest_product_does_not_use_the_smallest_factor() {
    let output = palindrome_products(3215, 4000);
    assert!(output.is_some());

    let (pal, _) = output.unwrap();
    assert_eq!(pal.value(), 10988901);
    assert_eq!(pal.into_factors(), HashSet::from([(3297, 3333)]));
}
