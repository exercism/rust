use sublist::{sublist, Comparison};

#[test]
fn empty_equals_empty() {
    let v: &[u32] = &[];

    assert_eq!(Comparison::Equal, sublist(v, v));
}

#[test]
#[ignore]
fn test_empty_is_a_sublist_of_anything() {
    assert_eq!(Comparison::Sublist, sublist(&[], &['a', 's', 'd', 'f']));
}

#[test]
#[ignore]
fn test_anything_is_a_superlist_of_empty() {
    assert_eq!(Comparison::Superlist, sublist(&['a', 's', 'd', 'f'], &[]));
}

#[test]
#[ignore]
fn test_1_is_not_2() {
    assert_eq!(Comparison::Unequal, sublist(&[1], &[2]));
}

#[test]
#[ignore]
fn test_compare_larger_equal_lists() {
    use std::iter::repeat;

    let v: Vec<char> = repeat('x').take(1000).collect();

    assert_eq!(Comparison::Equal, sublist(&v, &v));
}

#[test]
#[ignore]
fn test_sublist_at_start() {
    assert_eq!(Comparison::Sublist, sublist(&[1, 2, 3], &[1, 2, 3, 4, 5]));
}

#[test]
#[ignore]
fn sublist_in_middle() {
    assert_eq!(Comparison::Sublist, sublist(&[4, 3, 2], &[5, 4, 3, 2, 1]));
}

#[test]
#[ignore]
fn sublist_at_end() {
    assert_eq!(Comparison::Sublist, sublist(&[3, 4, 5], &[1, 2, 3, 4, 5]));
}

#[test]
#[ignore]
fn partially_matching_sublist_at_start() {
    assert_eq!(Comparison::Sublist, sublist(&[1, 1, 2], &[1, 1, 1, 2]));
}

#[test]
#[ignore]
fn sublist_early_in_huge_list() {
    let huge: Vec<u32> = (1..1_000_000).collect();

    assert_eq!(Comparison::Sublist, sublist(&[3, 4, 5], &huge));
}

#[test]
#[ignore]
fn huge_sublist_not_in_huge_list() {
    let v1: Vec<u64> = (10..1_000_001).collect();
    let v2: Vec<u64> = (1..1_000_000).collect();

    assert_eq!(Comparison::Unequal, sublist(&v1, &v2));
}

#[test]
#[ignore]
fn superlist_at_start() {
    assert_eq!(Comparison::Superlist, sublist(&[1, 2, 3, 4, 5], &[1, 2, 3]));
}

#[test]
#[ignore]
fn superlist_in_middle() {
    assert_eq!(Comparison::Superlist, sublist(&[5, 4, 3, 2, 1], &[4, 3, 2]));
}

#[test]
#[ignore]
fn superlist_at_end() {
    assert_eq!(Comparison::Superlist, sublist(&[1, 2, 3, 4, 5], &[3, 4, 5]));
}

#[test]
#[ignore]
fn second_list_missing_element_from_first_list() {
    assert_eq!(Comparison::Unequal, sublist(&[1, 2, 3], &[1, 3]));
}

#[test]
#[ignore]
fn superlist_early_in_huge_list() {
    let huge: Vec<u32> = (1..1_000_000).collect();

    assert_eq!(Comparison::Superlist, sublist(&huge, &[3, 4, 5]));
}

#[test]
#[ignore]
fn recurring_values_sublist() {
    assert_eq!(
        Comparison::Sublist,
        sublist(&[1, 2, 1, 2, 3], &[1, 2, 3, 1, 2, 1, 2, 3, 2, 1])
    );
}

#[test]
#[ignore]
fn recurring_values_unequal() {
    assert_eq!(
        Comparison::Unequal,
        sublist(&[1, 2, 1, 2, 3], &[1, 2, 3, 1, 2, 3, 2, 3, 2, 1])
    );
}
