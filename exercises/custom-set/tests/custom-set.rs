use custom_set::*;

#[test]
fn sets_with_no_elements_are_empty() {
    let set: CustomSet<()> = CustomSet::new(&[]);
    assert!(set.is_empty());
}

#[test]
#[ignore]
fn sets_with_elements_are_not_empty() {
    let set = CustomSet::new(&[1]);
    assert!(!set.is_empty());
}

#[test]
#[ignore]
fn nothing_is_contained_in_an_empty_set() {
    let set = CustomSet::new(&[]);
    assert!(!set.contains(&1));
}

#[test]
#[ignore]
fn true_when_the_element_is_in_the_set() {
    let set = CustomSet::new(&[1, 2, 3]);
    assert!(set.contains(&1));
}

#[test]
#[ignore]
fn false_when_the_element_is_not_in_the_set() {
    let set = CustomSet::new(&[1, 2, 3]);
    assert!(!set.contains(&4));
}

#[test]
#[ignore]
fn empty_sets_are_subsets_of_each_other() {
    let set1: CustomSet<()> = CustomSet::new(&[]);
    let set2: CustomSet<()> = CustomSet::new(&[]);
    assert!(set1.is_subset(&set2));
    assert!(set2.is_subset(&set1));
}

#[test]
#[ignore]
fn empty_set_is_subset_of_non_empty_set() {
    let set1 = CustomSet::new(&[]);
    let set2 = CustomSet::new(&[1]);
    assert!(set1.is_subset(&set2));
}

#[test]
#[ignore]
fn non_empty_set_is_not_subset_of_empty_set() {
    let set1 = CustomSet::new(&[1]);
    let set2 = CustomSet::new(&[]);
    assert!(!set1.is_subset(&set2));
}

#[test]
#[ignore]
fn sets_with_same_elements_are_subsets() {
    let set1 = CustomSet::new(&[1, 2, 3]);
    let set2 = CustomSet::new(&[1, 2, 3]);
    assert!(set1.is_subset(&set2));
    assert!(set2.is_subset(&set1));
}

#[test]
#[ignore]
fn set_contained_in_other_set_is_a_subset() {
    let set1 = CustomSet::new(&[1, 2, 3]);
    let set2 = CustomSet::new(&[4, 1, 2, 3]);
    assert!(set1.is_subset(&set2));
}

#[test]
#[ignore]
fn set_not_contained_in_other_set_is_not_a_subset_one() {
    let set1 = CustomSet::new(&[1, 2, 3]);
    let set2 = CustomSet::new(&[4, 1, 3]);
    assert!(!set1.is_subset(&set2));
}

#[test]
#[ignore]
fn empty_sets_are_disjoint_with_each_other() {
    let set1: CustomSet<()> = CustomSet::new(&[]);
    let set2: CustomSet<()> = CustomSet::new(&[]);
    assert!(set1.is_disjoint(&set2));
    assert!(set2.is_disjoint(&set1));
}

#[test]
#[ignore]
fn empty_set_disjoint_with_non_empty_set() {
    let set1 = CustomSet::new(&[]);
    let set2 = CustomSet::new(&[1]);
    assert!(set1.is_disjoint(&set2));
}

#[test]
#[ignore]
fn non_empty_set_disjoint_with_empty_set() {
    let set1 = CustomSet::new(&[1]);
    let set2 = CustomSet::new(&[]);
    assert!(set1.is_disjoint(&set2));
}

#[test]
#[ignore]
fn sets_with_one_element_in_common_are_not_disjoint() {
    let set1 = CustomSet::new(&[1, 2]);
    let set2 = CustomSet::new(&[2, 3]);
    assert!(!set1.is_disjoint(&set2));
    assert!(!set2.is_disjoint(&set1));
}

#[test]
#[ignore]
fn sets_with_no_elements_in_common_are_disjoint() {
    let set1 = CustomSet::new(&[1, 2]);
    let set2 = CustomSet::new(&[3, 4]);
    assert!(set1.is_disjoint(&set2));
    assert!(set2.is_disjoint(&set1));
}

#[test]
#[ignore]
fn empty_sets_are_equal() {
    let set1: CustomSet<()> = CustomSet::new(&[]);
    let set2: CustomSet<()> = CustomSet::new(&[]);
    assert_eq!(set1, set2);
}

#[test]
#[ignore]
fn empty_set_is_not_equal_to_a_non_empty_set() {
    let set1 = CustomSet::new(&[]);
    let set2 = CustomSet::new(&[1, 2, 3]);
    assert_ne!(set1, set2);
}

#[test]
#[ignore]
fn non_empty_set_is_not_equal_to_an_empty_set() {
    let set1 = CustomSet::new(&[1, 2, 3]);
    let set2 = CustomSet::new(&[]);
    assert_ne!(set1, set2);
}

#[test]
#[ignore]
fn sets_with_the_same_elements_are_equal() {
    let set1 = CustomSet::new(&[1, 2]);
    let set2 = CustomSet::new(&[2, 1]);
    assert_eq!(set1, set2);
}

#[test]
#[ignore]
fn sets_with_different_elements_are_not_equal() {
    let set1 = CustomSet::new(&[1, 2, 3]);
    let set2 = CustomSet::new(&[2, 1, 4]);
    assert_ne!(set1, set2);
}

#[test]
#[ignore]
fn add_to_empty_set() {
    let mut set = CustomSet::new(&[]);
    set.add(3);
    assert_eq!(set, CustomSet::new(&[3]));
}

#[test]
#[ignore]
fn add_to_non_empty_set() {
    let mut set = CustomSet::new(&[1, 2, 4]);
    set.add(3);
    assert_eq!(set, CustomSet::new(&[1, 2, 3, 4]));
}

#[test]
#[ignore]
fn add_existing_element() {
    let mut set = CustomSet::new(&[1, 2, 3]);
    set.add(3);
    assert_eq!(set, CustomSet::new(&[1, 2, 3]));
}

#[test]
#[ignore]
fn intersecting_empty_sets_return_empty_set() {
    let set1: CustomSet<()> = CustomSet::new(&[]);
    let set2: CustomSet<()> = CustomSet::new(&[]);
    assert_eq!(set1.intersection(&set2), CustomSet::new(&[]));
}

#[test]
#[ignore]
fn intersecting_empty_set_with_non_empty_returns_empty_set() {
    let set1 = CustomSet::new(&[]);
    let set2 = CustomSet::new(&[3, 2, 5]);
    assert_eq!(set1.intersection(&set2), CustomSet::new(&[]));
}

#[test]
#[ignore]
fn intersecting_non_empty_set_with_empty_returns_empty_set() {
    let set1 = CustomSet::new(&[1, 2, 3, 4]);
    let set2 = CustomSet::new(&[]);
    assert_eq!(set1.intersection(&set2), CustomSet::new(&[]));
}

#[test]
#[ignore]
fn intersection_of_two_sets_with_no_shared_elements_is_an_empty_set() {
    let set1 = CustomSet::new(&[1, 2, 3]);
    let set2 = CustomSet::new(&[4, 5, 6]);
    assert_eq!(set1.intersection(&set2), CustomSet::new(&[]));
    assert_eq!(set2.intersection(&set1), CustomSet::new(&[]));
}

#[test]
#[ignore]
fn intersection_of_two_sets_with_shared_elements_is_a_set_of_the_shared_elements() {
    let set1 = CustomSet::new(&[1, 2, 3, 4]);
    let set2 = CustomSet::new(&[3, 2, 5]);
    assert_eq!(set1.intersection(&set2), CustomSet::new(&[2, 3]));
    assert_eq!(set2.intersection(&set1), CustomSet::new(&[2, 3]));
}

#[test]
#[ignore]
fn difference_of_two_empty_sets_is_empty_set() {
    let set1: CustomSet<()> = CustomSet::new(&[]);
    let set2: CustomSet<()> = CustomSet::new(&[]);
    assert_eq!(set1.difference(&set2), CustomSet::new(&[]));
}

#[test]
#[ignore]
fn difference_of_an_empty_and_non_empty_set_is_an_empty_set() {
    let set1 = CustomSet::new(&[]);
    let set2 = CustomSet::new(&[3, 2, 5]);
    assert_eq!(set1.difference(&set2), CustomSet::new(&[]));
}

#[test]
#[ignore]
fn difference_of_a_non_empty_set_and_empty_set_is_the_non_empty_set() {
    let set1 = CustomSet::new(&[1, 2, 3, 4]);
    let set2 = CustomSet::new(&[]);
    assert_eq!(set1.difference(&set2), CustomSet::new(&[1, 2, 3, 4]));
}

#[test]
#[ignore]
fn difference_of_two_non_empty_sets_is_elements_only_in_first_set_one() {
    let set1 = CustomSet::new(&[3, 2, 1]);
    let set2 = CustomSet::new(&[2, 4]);
    assert_eq!(set1.difference(&set2), CustomSet::new(&[1, 3]));
}

#[test]
#[ignore]
fn union_of_two_empty_sets_is_empty_set() {
    let set1: CustomSet<()> = CustomSet::new(&[]);
    let set2: CustomSet<()> = CustomSet::new(&[]);
    assert_eq!(set1.union(&set2), CustomSet::new(&[]));
}

#[test]
#[ignore]
fn union_of_empty_set_and_non_empty_set_is_all_elements() {
    let set1 = CustomSet::new(&[]);
    let set2 = CustomSet::new(&[2]);
    assert_eq!(set1.union(&set2), CustomSet::new(&[2]));
}

#[test]
#[ignore]
fn union_of_non_empty_set_and_empty_set_is_the_non_empty_set() {
    let set1 = CustomSet::new(&[1, 3]);
    let set2 = CustomSet::new(&[]);
    assert_eq!(set1.union(&set2), CustomSet::new(&[1, 3]));
}

#[test]
#[ignore]
fn union_of_non_empty_sets_contains_all_unique_elements() {
    let set1 = CustomSet::new(&[1, 3]);
    let set2 = CustomSet::new(&[2, 3]);
    assert_eq!(set1.union(&set2), CustomSet::new(&[3, 2, 1]));
}
