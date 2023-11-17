// The &[] borrows are required for the base exercise,
// where `find` is not generic. Once `find` is made generic,
// the borrows become needless. Since we want the tests to work
// without clippy warnings for both people who take on the
// additional challenge and people who don't, we disable this lint.
#![allow(clippy::needless_borrows_for_generic_args)]

use binary_search::find;

#[test]
fn finds_a_value_in_an_array_with_one_element() {
    assert_eq!(find(&[6], 6), Some(0));
}

#[test]
#[ignore]
fn finds_first_value_in_an_array_with_two_element() {
    assert_eq!(find(&[1, 2], 1), Some(0));
}

#[test]
#[ignore]
fn finds_second_value_in_an_array_with_two_element() {
    assert_eq!(find(&[1, 2], 2), Some(1));
}

#[test]
#[ignore]
fn finds_a_value_in_the_middle_of_an_array() {
    assert_eq!(find(&[1, 3, 4, 6, 8, 9, 11], 6), Some(3));
}

#[test]
#[ignore]
fn finds_a_value_at_the_beginning_of_an_array() {
    assert_eq!(find(&[1, 3, 4, 6, 8, 9, 11], 1), Some(0));
}

#[test]
#[ignore]
fn finds_a_value_at_the_end_of_an_array() {
    assert_eq!(find(&[1, 3, 4, 6, 8, 9, 11], 11), Some(6));
}

#[test]
#[ignore]
fn finds_a_value_in_an_array_of_odd_length() {
    assert_eq!(
        find(&[1, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377, 634], 144),
        Some(9)
    );
}

#[test]
#[ignore]
fn finds_a_value_in_an_array_of_even_length() {
    assert_eq!(
        find(&[1, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377], 21),
        Some(5)
    );
}

#[test]
#[ignore]
fn identifies_that_a_value_is_not_included_in_the_array() {
    assert_eq!(find(&[1, 3, 4, 6, 8, 9, 11], 7), None);
}

#[test]
#[ignore]
fn a_value_smaller_than_the_arrays_smallest_value_is_not_included() {
    assert_eq!(find(&[1, 3, 4, 6, 8, 9, 11], 0), None);
}

#[test]
#[ignore]
fn a_value_larger_than_the_arrays_largest_value_is_not_included() {
    assert_eq!(find(&[1, 3, 4, 6, 8, 9, 11], 13), None);
}

#[test]
#[ignore]
fn nothing_is_included_in_an_empty_array() {
    assert_eq!(find(&[], 1), None);
}

#[test]
#[ignore]
fn nothing_is_found_when_the_left_and_right_bounds_cross() {
    assert_eq!(find(&[1, 2], 0), None);
}

#[test]
#[ignore]
#[cfg(feature = "generic")]
fn works_for_arrays() {
    assert_eq!(find([6], 6), Some(0));
}

#[test]
#[ignore]
#[cfg(feature = "generic")]
fn works_for_vec() {
    let vector = vec![6];
    assert_eq!(find(&vector, 6), Some(0));
    assert_eq!(find(vector, 6), Some(0));
}

#[test]
#[ignore]
#[cfg(feature = "generic")]
fn works_for_str_elements() {
    assert_eq!(find(["a"], "a"), Some(0));
    assert_eq!(find(["a", "b"], "b"), Some(1));
}
