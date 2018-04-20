extern crate palindrome_products;
use palindrome_products::*;

#[test]
fn single_digits() {
    let palindromes = get_palindrome_products(1, 9);
    assert_eq!(min(&palindromes), Some(1));
    assert_eq!(max(&palindromes), Some(9));
}

#[test]
#[ignore]
fn double_digits() {
    let palindromes = get_palindrome_products(10, 99);
    assert_eq!(min(&palindromes), Some(121));
    assert_eq!(max(&palindromes), Some(9009));
}

#[test]
#[ignore]
fn triple_digits() {
    let palindromes = get_palindrome_products(100, 999);
    assert_eq!(min(&palindromes), Some(10201));
    assert_eq!(max(&palindromes), Some(906609));
}

#[test]
#[ignore]
fn four_digits() {
    let palindromes = get_palindrome_products(1000, 9999);
    assert_eq!(min(&palindromes), Some(1002001));
    assert_eq!(max(&palindromes), Some(99000099));
}

#[test]
#[ignore]
fn empty_result_for_smallest_palindrome() {
    assert_eq!(min(&get_palindrome_products(1002, 1003)), None);
}

#[test]
#[ignore]
fn empty_result_for_largest_palindrome() {
    assert_eq!(max(&get_palindrome_products(15, 15)), None);
}

#[test]
#[ignore]
fn error_smallest_palindrome_when_min_gt_max() {
    assert_eq!(min(&get_palindrome_products(1000, 1)), None);
}

#[test]
#[ignore]
fn error_largest_palindrome_when_min_st_max() {
    assert_eq!(max(&get_palindrome_products(2, 1)), None);
}
