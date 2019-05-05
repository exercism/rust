use palindrome_products::*;

#[test]
fn single_digits() {
    let palindromes = get_palindrome_products(1, 9);
    assert_eq!(Some(1), min(&palindromes));
    assert_eq!(Some(9), max(&palindromes));
}

#[test]
#[ignore]
fn double_digits() {
    let palindromes = get_palindrome_products(10, 99);
    assert_eq!(Some(121), min(&palindromes));
    assert_eq!(Some(9009), max(&palindromes));
}

#[test]
#[ignore]
fn triple_digits() {
    let palindromes = get_palindrome_products(100, 999);
    assert_eq!(Some(10201), min(&palindromes));
    assert_eq!(Some(906609), max(&palindromes));
}

#[test]
#[ignore]
fn four_digits() {
    let palindromes = get_palindrome_products(1000, 9999);
    assert_eq!(Some(1002001), min(&palindromes));
    assert_eq!(Some(99000099), max(&palindromes));
}

#[test]
#[ignore]
fn empty_result_for_smallest_palindrome() {
    assert_eq!(None, min(&get_palindrome_products(1002, 1003)));
}

#[test]
#[ignore]
fn empty_result_for_largest_palindrome() {
    assert_eq!(None, max(&get_palindrome_products(15, 15)));
}

#[test]
#[ignore]
fn error_smallest_palindrome_when_min_gt_max() {
    assert_eq!(None, min(&get_palindrome_products(1000, 1)));
}

#[test]
#[ignore]
fn error_largest_palindrome_when_min_st_max() {
    assert_eq!(None, max(&get_palindrome_products(2, 1)));
}
