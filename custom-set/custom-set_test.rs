#![crate_name = "custom-set_test"]
#![crate_type = "lib"]

use std::collections::{Collection, Set, MutableSet};

mod set;

fn make_set(vec: Vec<uint>) -> set::CustomSet<uint> {
    vec.into_iter().collect()
}

#[test]
#[ignore]
fn test_empty_set() {
    let set = set::CustomSet::<()>::new();
    assert_eq!(set.len(), 0);
    assert_eq!(set.is_empty(), true);
}

#[test]
#[ignore]
fn test_from_iter() {
    let set = make_set(vec!(1, 2, 3, 2));
    assert_eq!(set.len(), 3);
    assert!(!set.is_empty());
    assert!(set.contains(&1));
    assert!(set.contains(&3));
    assert!(!set.contains(&4));
}

#[test]
#[ignore]
fn test_is_disjoint() {
    assert!(make_set(vec!()).is_disjoint(&make_set(vec!())));
    assert!(make_set(vec!(1)).is_disjoint(&make_set(vec!())));
    assert!(make_set(vec!()).is_disjoint(&make_set(vec!(1))));
    assert!(make_set(vec!(1, 2)).is_disjoint(&make_set(vec!(3, 4))));
    assert!(!make_set(vec!(1, 2)).is_disjoint(&make_set(vec!(2, 4))));
}

#[test]
#[ignore]
fn test_is_subset() {
    // Technically improper subset
    assert!(make_set(vec!()).is_subset(&make_set(vec!())));
    assert!(!make_set(vec!(1)).is_subset(&make_set(vec!())));
    assert!(make_set(vec!()).is_subset(&make_set(vec!(1))));
    assert!(!make_set(vec!(1, 2)).is_subset(&make_set(vec!(3, 4))));
    assert!(!make_set(vec!(1, 2)).is_subset(&make_set(vec!(2, 4))));
    assert!(make_set(vec!(1, 2)).is_subset(&make_set(vec!(1, 2, 4))));
}

#[test]
#[ignore]
fn test_is_superset() {
    assert!(make_set(vec!()).is_superset(&make_set(vec!())));
    assert!(make_set(vec!(1)).is_superset(&make_set(vec!())));
    assert!(!make_set(vec!()).is_superset(&make_set(vec!(1))));
    assert!(!make_set(vec!(1, 2)).is_superset(&make_set(vec!(3, 4))));
    assert!(!make_set(vec!(1, 2)).is_superset(&make_set(vec!(2, 4))));
    assert!(!make_set(vec!(1, 2)).is_superset(&make_set(vec!(1, 2, 4))));
    assert!(make_set(vec!(1, 2, 3)).is_superset(&make_set(vec!(1, 2))));
}

fn difference(a: Vec<uint>, b: Vec<uint>) -> Vec<uint> {
    make_set(a).difference(&make_set(b)).map(|n| n.clone()).collect()
}

#[test]
#[ignore]
fn test_difference() {
    assert_eq!(difference(vec!(), vec!()), vec!());
    assert_eq!(difference(vec!(), vec!(3, 2, 5)), vec!());
    assert_eq!(difference(vec!(1, 2, 3, 4), vec!()), vec!(1, 2, 3, 4));
    assert_eq!(difference(vec!(1, 2, 3, 4), vec!(3, 2, 5)), vec!(1, 4));
}

fn intersection(a: Vec<uint>, b: Vec<uint>) -> Vec<uint> {
    make_set(a).intersection(&make_set(b)).map(|n| n.clone()).collect()
}

#[test]
#[ignore]
fn test_intersection() {
    assert_eq!(intersection(vec!(), vec!()), vec!());
    assert_eq!(intersection(vec!(), vec!(3, 2, 5)), vec!());
    assert_eq!(intersection(vec!(1, 2, 3, 4), vec!()), vec!());
    assert_eq!(intersection(vec!(1, 2, 3, 4), vec!(3, 2, 5)), vec!(2, 3));
}

fn union(a: Vec<uint>, b: Vec<uint>) -> Vec<uint> {
    make_set(a).union(&make_set(b)).map(|n| n.clone()).collect()
}

#[test]
#[ignore]
fn test_union() {
    assert_eq!(union(vec!(), vec!()), vec!());
    assert_eq!(union(vec!(), vec!(3, 2, 5)), vec!(2, 3, 5));
    assert_eq!(union(vec!(1, 2, 3, 4), vec!()), vec!(1, 2, 3, 4));
    assert_eq!(union(vec!(1, 2, 3, 4), vec!(3, 2, 5)), vec!(1, 2, 3, 4, 5));
}

#[test]
#[ignore]
fn test_insert() {
    let mut set = make_set(vec!(1, 2, 3));
    assert!(set.contains(&2));
    assert!(!set.insert(2));
    assert!(set.contains(&2));
    assert!(!set.contains(&4));
    assert!(set.insert(4));
    assert!(set.contains(&4));
}

// Equality on this is modulo 3.
#[deriving(Eq, Show)]
struct Modulo3(uint);

impl PartialEq for Modulo3 {
    fn eq(&self, other: &Modulo3) -> bool {
        let &Modulo3(ref a) = self;
        let &Modulo3(ref b) = other;
        a % 3 == b % 3
    }
}

impl PartialOrd for Modulo3 {
    fn partial_cmp(&self, other: &Modulo3) -> Option<Ordering> {
        let &Modulo3(ref a) = self;
        let &Modulo3(ref b) = other;
        (a % 3).partial_cmp(&(b % 3))
    }
}

impl Ord for Modulo3 {
    fn cmp(&self, other: &Modulo3) -> Ordering {
        let &Modulo3(ref a) = self;
        let &Modulo3(ref b) = other;
        (a % 3).cmp(&(b % 3))
    }
}

#[test]
#[ignore]
fn test_insert_no_double() {
    // This test abuses the ord and eq mechanisms a bit to check that a set doesn't replace
    // existing elements with new elements, which could lead to interesting bugs if the programmer
    // is stupid enough to trigger that behaviour.
    let mut set = vec!(Modulo3(1)).into_iter().collect::<set::CustomSet<Modulo3>>();
    assert!(set.contains(&Modulo3(1)));
    assert!(set.contains(&Modulo3(4)));
    assert!(set.insert(Modulo3(2)));
    assert!(set.contains(&Modulo3(5)));
    assert!(!set.insert(Modulo3(8)));
    let v = set.iter().collect::<Vec<&Modulo3>>();
    let &Modulo3(ref v0) = v[0];
    let &Modulo3(ref v1) = v[1];
    assert_eq!(v0, &1);
    assert_eq!(v1, &2);
}

#[test]
#[ignore]
fn test_remove() {
    let mut set = make_set(vec!(1, 2, 3));
    assert!(set.contains(&2));
    assert!(set.remove(&2));
    assert!(!set.contains(&2));
    assert!(!set.remove(&4));
    assert!(!set.contains(&4));
}

#[test]
#[ignore]
fn test_clear() {
    let mut set = make_set(vec!(1, 2, 3));
    assert!(set.contains(&2));
    assert_eq!(set.len(), 3);
    assert!(!set.is_empty());
    set.clear();
    assert!(!set.contains(&2));
    assert_eq!(set.len(), 0);
    assert!(set.is_empty());
}

#[test]
#[ignore]
fn test_traits() {
    let s: set::CustomSet<()> = set::CustomSet::<()>::new();
    let _ = &s as &Collection;
    let _ = &s as &Set<()>;
    let _ = &s as &MutableSet<()>;
}
