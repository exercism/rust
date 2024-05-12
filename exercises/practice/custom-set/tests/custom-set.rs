use custom_set::CustomSet;

#[test]
fn sets_with_no_elements_are_empty() {
    let set = CustomSet::<i32>::new(&[]);
    assert!(set.is_empty());
}

#[test]
#[ignore]
fn sets_with_elements_are_not_empty() {
    let set = CustomSet::<i32>::new(&[1]);
    assert!(!set.is_empty());
}

#[test]
#[ignore]
fn nothing_is_contained_in_an_empty_set() {
    let set = CustomSet::<i32>::new(&[]);
    assert!(!set.contains(&1));
}

#[test]
#[ignore]
fn when_the_element_is_in_the_set() {
    let set = CustomSet::<i32>::new(&[1, 2, 3]);
    assert!(set.contains(&1));
}

#[test]
#[ignore]
fn when_the_element_is_not_in_the_set() {
    let set = CustomSet::<i32>::new(&[1, 2, 3]);
    assert!(!set.contains(&4));
}

#[test]
#[ignore]
fn empty_set_is_a_subset_of_another_empty_set() {
    let set_1 = CustomSet::<i32>::new(&[]);
    let set_2 = CustomSet::<i32>::new(&[]);
    assert!(set_1.is_subset(&set_2));
}

#[test]
#[ignore]
fn empty_set_is_a_subset_of_non_empty_set() {
    let set_1 = CustomSet::<i32>::new(&[]);
    let set_2 = CustomSet::<i32>::new(&[1]);
    assert!(set_1.is_subset(&set_2));
}

#[test]
#[ignore]
fn non_empty_set_is_not_a_subset_of_empty_set() {
    let set_1 = CustomSet::<i32>::new(&[1]);
    let set_2 = CustomSet::<i32>::new(&[]);
    assert!(!set_1.is_subset(&set_2));
}

#[test]
#[ignore]
fn set_is_a_subset_of_set_with_exact_same_elements() {
    let set_1 = CustomSet::<i32>::new(&[1, 2, 3]);
    let set_2 = CustomSet::<i32>::new(&[1, 2, 3]);
    assert!(set_1.is_subset(&set_2));
}

#[test]
#[ignore]
fn set_is_a_subset_of_larger_set_with_same_elements() {
    let set_1 = CustomSet::<i32>::new(&[1, 2, 3]);
    let set_2 = CustomSet::<i32>::new(&[4, 1, 2, 3]);
    assert!(set_1.is_subset(&set_2));
}

#[test]
#[ignore]
fn set_is_not_a_subset_of_set_that_does_not_contain_its_elements() {
    let set_1 = CustomSet::<i32>::new(&[1, 2, 3]);
    let set_2 = CustomSet::<i32>::new(&[4, 1, 3]);
    assert!(!set_1.is_subset(&set_2));
}

#[test]
#[ignore]
fn the_empty_set_is_disjoint_with_itself() {
    let set_1 = CustomSet::<i32>::new(&[]);
    let set_2 = CustomSet::<i32>::new(&[]);
    assert!(set_1.is_disjoint(&set_2));
}

#[test]
#[ignore]
fn empty_set_is_disjoint_with_non_empty_set() {
    let set_1 = CustomSet::<i32>::new(&[]);
    let set_2 = CustomSet::<i32>::new(&[1]);
    assert!(set_1.is_disjoint(&set_2));
}

#[test]
#[ignore]
fn non_empty_set_is_disjoint_with_empty_set() {
    let set_1 = CustomSet::<i32>::new(&[1]);
    let set_2 = CustomSet::<i32>::new(&[]);
    assert!(set_1.is_disjoint(&set_2));
}

#[test]
#[ignore]
fn sets_are_not_disjoint_if_they_share_an_element() {
    let set_1 = CustomSet::<i32>::new(&[1, 2]);
    let set_2 = CustomSet::<i32>::new(&[2, 3]);
    assert!(!set_1.is_disjoint(&set_2));
}

#[test]
#[ignore]
fn sets_are_disjoint_if_they_share_no_elements() {
    let set_1 = CustomSet::<i32>::new(&[1, 2]);
    let set_2 = CustomSet::<i32>::new(&[3, 4]);
    assert!(set_1.is_disjoint(&set_2));
}

#[test]
#[ignore]
fn empty_sets_are_equal() {
    let set_1 = CustomSet::<i32>::new(&[]);
    let set_2 = CustomSet::<i32>::new(&[]);
    assert_eq!(set_1, set_2);
}

#[test]
#[ignore]
fn empty_set_is_not_equal_to_non_empty_set() {
    let set_1 = CustomSet::<i32>::new(&[]);
    let set_2 = CustomSet::<i32>::new(&[1, 2, 3]);
    assert_ne!(set_1, set_2);
}

#[test]
#[ignore]
fn non_empty_set_is_not_equal_to_empty_set() {
    let set_1 = CustomSet::<i32>::new(&[1, 2, 3]);
    let set_2 = CustomSet::<i32>::new(&[]);
    assert_ne!(set_1, set_2);
}

#[test]
#[ignore]
fn sets_with_the_same_elements_are_equal() {
    let set_1 = CustomSet::<i32>::new(&[1, 2]);
    let set_2 = CustomSet::<i32>::new(&[2, 1]);
    assert_eq!(set_1, set_2);
}

#[test]
#[ignore]
fn sets_with_different_elements_are_not_equal() {
    let set_1 = CustomSet::<i32>::new(&[1, 2, 3]);
    let set_2 = CustomSet::<i32>::new(&[1, 2, 4]);
    assert_ne!(set_1, set_2);
}

#[test]
#[ignore]
fn set_is_not_equal_to_larger_set_with_same_elements() {
    let set_1 = CustomSet::<i32>::new(&[1, 2, 3]);
    let set_2 = CustomSet::<i32>::new(&[1, 2, 3, 4]);
    assert_ne!(set_1, set_2);
}

#[test]
#[ignore]
fn set_is_equal_to_a_set_constructed_from_an_array_with_duplicates() {
    let set_1 = CustomSet::<i32>::new(&[1]);
    let set_2 = CustomSet::<i32>::new(&[1, 1]);
    assert_eq!(set_1, set_2);
}

#[test]
#[ignore]
fn add_to_empty_set() {
    let mut set = CustomSet::<i32>::new(&[]);
    set.add(3);
    let expected = CustomSet::<i32>::new(&[3]);
    assert_eq!(set, expected);
}

#[test]
#[ignore]
fn add_to_non_empty_set() {
    let mut set = CustomSet::<i32>::new(&[1, 2, 4]);
    set.add(3);
    let expected = CustomSet::<i32>::new(&[1, 2, 3, 4]);
    assert_eq!(set, expected);
}

#[test]
#[ignore]
fn adding_an_existing_element_does_not_change_the_set() {
    let mut set = CustomSet::<i32>::new(&[1, 2, 3]);
    set.add(3);
    let expected = CustomSet::<i32>::new(&[1, 2, 3]);
    assert_eq!(set, expected);
}

#[test]
#[ignore]
fn intersection_of_two_empty_sets_is_an_empty_set() {
    let set_1 = CustomSet::<i32>::new(&[]);
    let set_2 = CustomSet::<i32>::new(&[]);
    let expected = CustomSet::<i32>::new(&[]);
    assert_eq!(set_1.intersection(&set_2), expected);
}

#[test]
#[ignore]
fn intersection_of_an_empty_set_and_non_empty_set_is_an_empty_set() {
    let set_1 = CustomSet::<i32>::new(&[]);
    let set_2 = CustomSet::<i32>::new(&[3, 2, 5]);
    let expected = CustomSet::<i32>::new(&[]);
    assert_eq!(set_1.intersection(&set_2), expected);
}

#[test]
#[ignore]
fn intersection_of_a_non_empty_set_and_an_empty_set_is_an_empty_set() {
    let set_1 = CustomSet::<i32>::new(&[1, 2, 3, 4]);
    let set_2 = CustomSet::<i32>::new(&[]);
    let expected = CustomSet::<i32>::new(&[]);
    assert_eq!(set_1.intersection(&set_2), expected);
}

#[test]
#[ignore]
fn intersection_of_two_sets_with_no_shared_elements_is_an_empty_set() {
    let set_1 = CustomSet::<i32>::new(&[1, 2, 3]);
    let set_2 = CustomSet::<i32>::new(&[4, 5, 6]);
    let expected = CustomSet::<i32>::new(&[]);
    assert_eq!(set_1.intersection(&set_2), expected);
}

#[test]
#[ignore]
fn intersection_of_two_sets_with_shared_elements_is_a_set_of_the_shared_elements() {
    let set_1 = CustomSet::<i32>::new(&[1, 2, 3, 4]);
    let set_2 = CustomSet::<i32>::new(&[3, 2, 5]);
    let expected = CustomSet::<i32>::new(&[2, 3]);
    assert_eq!(set_1.intersection(&set_2), expected);
}

#[test]
#[ignore]
fn difference_of_two_empty_sets_is_an_empty_set() {
    let set_1 = CustomSet::<i32>::new(&[]);
    let set_2 = CustomSet::<i32>::new(&[]);
    let expected = CustomSet::<i32>::new(&[]);
    assert_eq!(set_1.difference(&set_2), expected);
}

#[test]
#[ignore]
fn difference_of_empty_set_and_non_empty_set_is_an_empty_set() {
    let set_1 = CustomSet::<i32>::new(&[]);
    let set_2 = CustomSet::<i32>::new(&[3, 2, 5]);
    let expected = CustomSet::<i32>::new(&[]);
    assert_eq!(set_1.difference(&set_2), expected);
}

#[test]
#[ignore]
fn difference_of_a_non_empty_set_and_an_empty_set_is_the_non_empty_set() {
    let set_1 = CustomSet::<i32>::new(&[1, 2, 3, 4]);
    let set_2 = CustomSet::<i32>::new(&[]);
    let expected = CustomSet::<i32>::new(&[1, 2, 3, 4]);
    assert_eq!(set_1.difference(&set_2), expected);
}

#[test]
#[ignore]
fn difference_of_two_non_empty_sets_is_a_set_of_elements_that_are_only_in_the_first_set() {
    let set_1 = CustomSet::<i32>::new(&[3, 2, 1]);
    let set_2 = CustomSet::<i32>::new(&[2, 4]);
    let expected = CustomSet::<i32>::new(&[1, 3]);
    assert_eq!(set_1.difference(&set_2), expected);
}

#[test]
#[ignore]
fn difference_removes_all_duplicates_in_the_first_set() {
    let set_1 = CustomSet::<i32>::new(&[1, 1]);
    let set_2 = CustomSet::<i32>::new(&[1]);
    let expected = CustomSet::<i32>::new(&[]);
    assert_eq!(set_1.difference(&set_2), expected);
}

#[test]
#[ignore]
fn union_of_empty_sets_is_an_empty_set() {
    let set_1 = CustomSet::<i32>::new(&[]);
    let set_2 = CustomSet::<i32>::new(&[]);
    let expected = CustomSet::<i32>::new(&[]);
    assert_eq!(set_1.union(&set_2), expected);
}

#[test]
#[ignore]
fn union_of_an_empty_set_and_non_empty_set_is_the_non_empty_set() {
    let set_1 = CustomSet::<i32>::new(&[]);
    let set_2 = CustomSet::<i32>::new(&[2]);
    let expected = CustomSet::<i32>::new(&[2]);
    assert_eq!(set_1.union(&set_2), expected);
}

#[test]
#[ignore]
fn union_of_a_non_empty_set_and_empty_set_is_the_non_empty_set() {
    let set_1 = CustomSet::<i32>::new(&[1, 3]);
    let set_2 = CustomSet::<i32>::new(&[]);
    let expected = CustomSet::<i32>::new(&[1, 3]);
    assert_eq!(set_1.union(&set_2), expected);
}

#[test]
#[ignore]
fn union_of_non_empty_sets_contains_all_unique_elements() {
    let set_1 = CustomSet::<i32>::new(&[1, 3]);
    let set_2 = CustomSet::<i32>::new(&[2, 3]);
    let expected = CustomSet::<i32>::new(&[3, 2, 1]);
    assert_eq!(set_1.union(&set_2), expected);
}
