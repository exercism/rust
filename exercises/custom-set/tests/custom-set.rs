extern crate custom_set as set;

use set::*;

#[test]
fn empty_sets_are_equal() {
    let set1: CustomSet<()> = CustomSet::new(vec![]);
    let set2: CustomSet<()> = CustomSet::new(vec![]);
    assert_eq!(set1, set2);
}

#[test]
fn sets_with_same_single_element_are_equal() {
    let set1 = CustomSet::new(vec![1]);
    let set2 = CustomSet::new(vec![1]);
    assert_eq!(set1, set2);
}

#[test]
fn duplicates_do_not_matter() {
    let set1 = CustomSet::new(vec![1, 1]);
    let set2 = CustomSet::new(vec![1]);
    assert_eq!(set1, set2);
}

#[test]
fn order_does_not_matter() {
    let set1 = CustomSet::new(vec![1, 3]);
    let set2 = CustomSet::new(vec![3, 1]);
    assert_eq!(set1, set2);
}

#[test]
fn different_sets_are_unequal() {
    let set1 = CustomSet::new(vec![1, 2, 3]);
    let set2 = CustomSet::new(vec![3, 4, 5]);
    assert!(set1 != set2);
}

#[test]
fn empty_set_is_not_equal_to_a_non_empty_set() {
    let set1 = CustomSet::new(vec![]);
    let set2 = CustomSet::new(vec![1, 2, 3]);
    assert!(set1 != set2);
}

#[test]
fn non_empty_set_is_not_equal_to_a_empty_set() {
    let set1 = CustomSet::new(vec![1, 2, 3]);
    let set2 = CustomSet::new(vec![]);
    assert!(set1 != set2);
}

#[test]
fn all_elements_must_be_equal_for_sets_to_be_equal() {
    let set1 = CustomSet::new(vec![1, 2, 3, 4]);
    let set2 = CustomSet::new(vec![2, 3, 4, 5]);
    assert!(set1 != set2);
}

#[test]
fn add_to_empty_set() {
    let mut set = CustomSet::new(vec![]);
    set.add(3);
    assert_eq!(set, CustomSet::new(vec![3]));
}

#[test]
fn add_to_non_empty_set() {
    let mut set = CustomSet::new(vec![1, 2, 4]);
    set.add(3);
    assert_eq!(set, CustomSet::new(vec![1, 2, 3, 4]));
}

#[test]
fn add_existing_element() {
    let mut set = CustomSet::new(vec![1, 2, 3]);
    set.add(3);
    assert_eq!(set, CustomSet::new(vec![1, 2, 3]));
}

#[test]
fn delete_an_existing_element() {
    let mut set = CustomSet::new(vec![3, 2, 1]);
    set.delete(&2);
    assert_eq!(set, CustomSet::new(vec![1, 3]));
}

#[test]
fn delete_an_nonexistent_element() {
    let mut set = CustomSet::new(vec![3, 2, 1]);
    set.delete(&4);
    assert_eq!(set, CustomSet::new(vec![1, 2, 3]));
}

#[test]
fn is_empty_without_elements() {
    let set: CustomSet<()> = CustomSet::new(vec![]);
    assert!(set.is_empty());
}

#[test]
fn is_not_empty_with_element() {
    let set = CustomSet::new(vec![1]);
    assert!(!set.is_empty());
}

#[test]
fn is_not_empty_with_elements() {
    let set = CustomSet::new(vec![1, 2, 3, 2]);
    assert!(!set.is_empty());
}

#[test]
fn empty_set_has_size_0() {
    let set: CustomSet<()> = CustomSet::new(vec![]);
    assert_eq!(set.size(), 0);
}

#[test]
fn non_empty_set_has_size_of_element_count() {
    let set = CustomSet::new(vec![1, 2, 4]);
    assert_eq!(set.size(), 3);
}

#[test]
fn duplicate_elements_are_not_counted() {
    let set = CustomSet::new(vec![1, 2, 3, 2]);
    assert_eq!(set.size(), 3);
}

#[test]
fn empty_set_does_not_contain_element() {
    let set = CustomSet::new(vec![]);
    assert!(!set.contains(&1));
}

#[test]
fn contains_element_1_in_set() {
    let set = CustomSet::new(vec![1, 2, 3, 2]);
    assert!(set.contains(&1));
}

#[test]
fn contains_element_2_in_set() {
    let set = CustomSet::new(vec![1, 2, 3, 2]);
    assert!(set.contains(&2));
}

#[test]
fn contains_element_3_in_set() {
    let set = CustomSet::new(vec![1, 2, 3, 2]);
    assert!(set.contains(&3));
}

#[test]
fn does_not_contain_element_4_in_set() {
    let set = CustomSet::new(vec![1, 2, 3, 2]);
    assert!(!set.contains(&4));
}

#[test]
fn empty_sets_are_subsets_of_each_other() {
    let set1: CustomSet<()> = CustomSet::new(vec![]);
    let set2: CustomSet<()> = CustomSet::new(vec![]);
    assert!(set1.is_subset(&set2));
    assert!(set2.is_subset(&set1));
}

#[test]
fn empty_set_is_subset_of_non_empty_set() {
    let set1 = CustomSet::new(vec![]);
    let set2 = CustomSet::new(vec![1]);
    assert!(set1.is_subset(&set2));
}

#[test]
fn non_empty_set_is_not_subset_of_empty_set() {
    let set1 = CustomSet::new(vec![]);
    let set2 = CustomSet::new(vec![1]);
    assert!(!set2.is_subset(&set1));
}

#[test]
fn sets_with_same_elements_are_subsets() {
    let set1 = CustomSet::new(vec![1, 2, 3]);
    let set2 = CustomSet::new(vec![1, 2, 3]);
    assert!(set1.is_subset(&set2));
    assert!(set2.is_subset(&set1));
}

#[test]
fn set_contained_in_other_set_is_a_subset() {
    let set1 = CustomSet::new(vec![1, 2, 3]);
    let set2 = CustomSet::new(vec![1, 2, 3, 4]);
    assert!(set1.is_subset(&set2));
}

#[test]
fn set_not_contained_in_other_set_is_not_a_subset_one() {
    let set1 = CustomSet::new(vec![1, 2, 3]);
    let set2 = CustomSet::new(vec![4, 1, 3]);
    assert!(!set1.is_subset(&set2));
}

#[test]
fn set_not_contained_in_other_set_is_not_a_subset_two() {
    let set1 = CustomSet::new(vec![1, 2, 3, 4, 5]);
    let set2 = CustomSet::new(vec![2, 3, 4]);
    assert!(!set1.is_subset(&set2));
}

#[test]
fn set_not_contained_in_other_set_is_not_a_subset_three() {
    let set1 = CustomSet::new(vec![1, 2, 3, 11]);
    let set2 = CustomSet::new(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    assert!(!set1.is_subset(&set2));
}

#[test]
fn empty_sets_are_disjoint_with_each_other() {
    let set1: CustomSet<()> = CustomSet::new(vec![]);
    let set2: CustomSet<()> = CustomSet::new(vec![]);
    assert!(set1.is_disjoint(&set2));
    assert!(set2.is_disjoint(&set1));
}

#[test]
fn empty_set_disjoint_with_non_empty_set() {
    let set1 = CustomSet::new(vec![]);
    let set2 = CustomSet::new(vec![1]);
    assert!(set1.is_disjoint(&set2));
}

#[test]
fn non_empty_set_disjoint_with_empty_set() {
    let set1 = CustomSet::new(vec![1]);
    let set2 = CustomSet::new(vec![]);
    assert!(set1.is_disjoint(&set2));
}

#[test]
fn sets_with_one_element_in_common_are_not_disjoint() {
    let set1 = CustomSet::new(vec![1, 2]);
    let set2 = CustomSet::new(vec![2, 3]);
    assert!(!set1.is_disjoint(&set2));
    assert!(!set2.is_disjoint(&set1));
}

#[test]
fn sets_with_no_elements_in_common_are_disjoint() {
    let set1 = CustomSet::new(vec![1, 2]);
    let set2 = CustomSet::new(vec![3, 4]);
    assert!(set1.is_disjoint(&set2));
    assert!(set2.is_disjoint(&set1));
}

#[test]
fn intersecting_empty_sets_return_empty_set() {
    let set1: CustomSet<()> = CustomSet::new(vec![]);
    let set2: CustomSet<()> = CustomSet::new(vec![]);
    assert_eq!(set1.intersection(&set2), CustomSet::new(vec![]));
}

#[test]
fn intersecting_empty_set_with_non_empty_returns_empty_set() {
    let set1 = CustomSet::new(vec![]);
    let set2 = CustomSet::new(vec![3, 2, 5]);
    assert_eq!(set1.intersection(&set2), CustomSet::new(vec![]));
}

#[test]
fn intersecting_non_empty_set_with_empty_returns_empty_set() {
    let set1 = CustomSet::new(vec![3, 2, 5]);
    let set2 = CustomSet::new(vec![]);
    assert_eq!(set1.intersection(&set2), CustomSet::new(vec![]));
}

#[test]
fn intersecting_equal_sets_returns_the_same_set() {
    let set1 = CustomSet::new(vec![3]);
    let set2 = CustomSet::new(vec![3]);
    assert_eq!(set1.intersection(&set2), CustomSet::new(vec![3]));
}

#[test]
fn intersecting_returns_shared_elements_one() {
    let set1 = CustomSet::new(vec![1, 2, 3]);
    let set2 = CustomSet::new(vec![3, 5, 4]);
    assert_eq!(set1.intersection(&set2), CustomSet::new(vec![3]));
}

#[test]
fn intersecting_returns_shared_elements_two() {
    let set1 = CustomSet::new(vec![1, 2, 3, 4]);
    let set2 = CustomSet::new(vec![3, 5, 4]);
    assert_eq!(set1.intersection(&set2), CustomSet::new(vec![3, 4]));
}

#[test]
fn intersecting_returns_shared_elements_three() {
    let set1 = CustomSet::new(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    let set2 = CustomSet::new(vec![5, 6, 7, 8, 9, 10]);
    assert_eq!(set1.intersection(&set2),
               CustomSet::new(vec![5, 6, 7, 8, 9, 10]));
}

#[test]
fn intersecting_no_shared_elements_returns_empty_set() {
    let set1 = CustomSet::new(vec![1, 2, 3]);
    let set2 = CustomSet::new(vec![4, 5, 6]);
    assert_eq!(set1.intersection(&set2), CustomSet::new(vec![]));
}

#[test]
fn union_of_two_empty_sets_is_empty_set() {
    let set1: CustomSet<()> = CustomSet::new(vec![]);
    let set2: CustomSet<()> = CustomSet::new(vec![]);
    assert_eq!(set1.union(&set2), CustomSet::new(vec![]));
}

#[test]
fn union_of_empty_set_and_non_empty_set_is_all_elements() {
    let set1 = CustomSet::new(vec![]);
    let set2 = CustomSet::new(vec![2]);
    assert_eq!(set1.union(&set2), CustomSet::new(vec![2]));
}

#[test]
fn union_of_empty_set_and_non_empty_set_is_all_elements_two() {
    let set1 = CustomSet::new(vec![]);
    let set2 = CustomSet::new(vec![3, 2, 5]);
    assert_eq!(set1.union(&set2), CustomSet::new(vec![3, 2, 5]));
}

#[test]
fn union_of_non_empty_set_and_empty_set_is_all_elements() {
    let set1 = CustomSet::new(vec![1, 3]);
    let set2 = CustomSet::new(vec![]);
    assert_eq!(set1.union(&set2), CustomSet::new(vec![1, 3]));
}

#[test]
fn union_of_two_sets_with_same_elements_contains_no_duplicates() {
    let set1 = CustomSet::new(vec![1, 3]);
    let set2 = CustomSet::new(vec![3, 1]);
    assert_eq!(set1.union(&set2), CustomSet::new(vec![1, 3]));
}

#[test]
fn union_of_two_non_empty_sets_contains_all_unique_elements_one() {
    let set1 = CustomSet::new(vec![1, 3]);
    let set2 = CustomSet::new(vec![2]);
    assert_eq!(set1.union(&set2), CustomSet::new(vec![1, 2, 3]));
}

#[test]
fn union_of_two_non_empty_sets_contains_all_unique_elements_two() {
    let set1 = CustomSet::new(vec![1, 3]);
    let set2 = CustomSet::new(vec![2, 3]);
    assert_eq!(set1.union(&set2), CustomSet::new(vec![1, 2, 3]));
}

#[test]
fn union_of_two_non_empty_sets_contains_all_unique_elements_three() {
    let set1 = CustomSet::new(vec![1, 2, 3, 4]);
    let set2 = CustomSet::new(vec![3, 2, 5]);
    assert_eq!(set1.union(&set2), CustomSet::new(vec![1, 2, 3, 4, 5]));
}

#[test]
fn difference_of_two_empty_sets_is_empty_set() {
    let set1: CustomSet<()> = CustomSet::new(vec![]);
    let set2: CustomSet<()> = CustomSet::new(vec![]);
    assert_eq!(set1.difference(&set2), CustomSet::new(vec![]));
}

#[test]
fn difference_of_an_empty_and_non_empty_set_is_an_empty_set() {
    let set1 = CustomSet::new(vec![]);
    let set2 = CustomSet::new(vec![3, 2, 5]);
    assert_eq!(set1.difference(&set2), CustomSet::new(vec![]));
}

#[test]
fn difference_of_a_non_empty_set_and_empty_set_is_the_non_empty_set() {
    let set1 = CustomSet::new(vec![1, 2, 3, 4]);
    let set2 = CustomSet::new(vec![]);
    assert_eq!(set1.difference(&set2), CustomSet::new(vec![1, 2, 3, 4]));
}

#[test]
fn difference_of_two_non_empty_sets_are_elements_only_in_first_set_one() {
    let set1 = CustomSet::new(vec![1, 2, 3]);
    let set2 = CustomSet::new(vec![4]);
    assert_eq!(set1.difference(&set2), CustomSet::new(vec![1, 2, 3]));
}

#[test]
fn difference_of_two_non_empty_sets_are_elements_only_in_first_set_two() {
    let set1 = CustomSet::new(vec![1, 2, 3]);
    let set2 = CustomSet::new(vec![4, 2]);
    assert_eq!(set1.difference(&set2), CustomSet::new(vec![1, 3]));
}

#[test]
fn difference_of_two_non_empty_sets_are_elements_only_in_first_set_three() {
    let set1 = CustomSet::new(vec![1, 2, 3, 4]);
    let set2 = CustomSet::new(vec![3, 2, 5]);
    assert_eq!(set1.difference(&set2), CustomSet::new(vec![1, 4]));
}

#[test]
fn symmetric_difference_of_two_empty_sets_is_an_empty_set() {
    let set1: CustomSet<()> = CustomSet::new(vec![]);
    let set2: CustomSet<()> = CustomSet::new(vec![]);
    assert_eq!(set1.symmetric_difference(&set2), CustomSet::new(vec![]));
}

#[test]
fn symmetric_difference_of_empty_and_non_empty_set_is_the_non_empty_set() {
    let set1 = CustomSet::new(vec![]);
    let set2 = CustomSet::new(vec![3, 2, 5]);
    assert_eq!(set1.symmetric_difference(&set2),
               CustomSet::new(vec![3, 2, 5]));
}

#[test]
fn symmetric_difference_of_non_empty_and_empty_set_is_the_non_empty_set() {
    let set1 = CustomSet::new(vec![]);
    let set2 = CustomSet::new(vec![1, 2, 3, 4]);
    assert_eq!(set1.symmetric_difference(&set2),
               CustomSet::new(vec![1, 2, 3, 4]));
}

#[test]
fn symmetric_difference_of_two_sets_is_unshared_elements_one() {
    let set1 = CustomSet::new(vec![1, 2, 3]);
    let set2 = CustomSet::new(vec![4]);
    assert_eq!(set1.symmetric_difference(&set2),
               CustomSet::new(vec![1, 2, 3, 4]));
}

#[test]
fn symmetric_difference_of_two_sets_is_unshared_elements_two() {
    let set1 = CustomSet::new(vec![3, 2, 1]);
    let set2 = CustomSet::new(vec![2, 4]);
    assert_eq!(set1.symmetric_difference(&set2),
               CustomSet::new(vec![1, 3, 4]));
}
