extern crate palindrome_products;
use palindrome_products::*;

#[test]
fn smallest_palindrome_single_digits(){
    assert_eq!(get_palindrome_product(1,9), Ok(1));
}

#[test]
fn largest_palindrome_single_digits(){
    assert_eq!(get_palindrome_product(1,9), Ok(9));
}

#[test]
fn smallest_palindrome_double_digits(){
    assert_eq!(get_palindrome_product(10,99), Ok(121));
}

#[test]
fn largest_palindrome_double_digits(){
    assert_eq!(get_palindrome_product(10,99), Ok(9009));
}

#[test]
fn smallest_palindrome_triple_digits(){
    assert_eq!(get_palindrome_product(100,999), Ok(10201));
}

#[test]
fn largest_palindrome_triple_digits(){
    assert_eq!(get_palindrome_product(100,999), Ok(906609));
}

#[test]
fn smallest_palindrome_four_digits(){
    assert_eq!(get_palindrome_product(1000,9999), Ok(1002001));
}

#[test]
fn largest_palindrome_four_digits(){
    assert_eq!(get_palindrome_product(1000,9999), Ok(99000099));
}

#[test]
fn empty_result_for_smallest_palindrome(){
    assert_eq!(get_palindrome_product(1002, 1003), Err(Error::Empty));
}

#[test]
fn empty_result_for_largest_palindrome(){
    assert_eq!(get_palindrome_product(15, 15), Err(Error::Empty));
}

#[test]
fn error_smallest_palindrome_when_min_bt_max(){
    assert_eq!(get_palindrome_product(1000, 1), Err(Error::RangeFailure));
}

#[test]
fn error_largest_palindrome_when_min_bt_max(){
    assert_eq!(get_palindrome_product(2, 1), Err(Error::RangeFailure));
}