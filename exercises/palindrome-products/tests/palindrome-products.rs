extern crate palindrome_products;
use palindrome_products::*;

#[test]
#[ignore]
fn empty_result_for_smallest_palindrome(){
    assert_eq!(min(get_palindrome_products(1002, 1003)), None);
}

#[test]
#[ignore]
fn empty_result_for_largest_palindrome(){
    assert_eq!(max(get_palindrome_products(15, 15)), None);
}

#[test]
#[ignore]
fn error_smallest_palindrome_when_min_gt_max(){
    assert_eq!(min(get_palindrome_products(1000, 1)), None);
}

#[test]
#[ignore]
fn error_largest_palindrome_when_min_st_max(){
    assert_eq!(max(get_palindrome_products(2, 1)), None);
}

#[test]
fn smallest_palindrome_single_digits(){
    assert_eq!(min(get_palindrome_products(1, 9)), Some(1));
}

#[test]
#[ignore]
fn largest_palindrome_single_digits(){
    assert_eq!(max(get_palindrome_products(1, 9)), Some(9));
}
#[test]
#[ignore]
fn smallest_palindrome_double_digits(){
    assert_eq!(min(get_palindrome_products(10, 99)), Some(121));
}

#[test]
#[ignore]
fn largest_palindrome_double_digits(){
    assert_eq!(max(get_palindrome_products(10, 99)), Some(9009));
}

#[test]
#[ignore]
fn smallest_palindrome_triple_digits(){
    assert_eq!(min(get_palindrome_products(100, 999)), Some(10201));
}

#[test]
#[ignore]
fn largest_palindrome_triple_digits(){
    assert_eq!(max(get_palindrome_products(100, 999)), Some(906609));
}

#[test]
#[ignore]
fn smallest_palindrome_four_digits(){
    assert_eq!(min(get_palindrome_products(1000, 9999)), Some(1002001));
}

#[test]
#[ignore]
fn largest_palindrome_four_digits(){
    assert_eq!(max(get_palindrome_products(1000, 9999)), Some(99000099));
}