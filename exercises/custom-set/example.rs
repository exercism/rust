use std::iter::{IntoIterator, FromIterator};
use std::slice;
use std::option::Option;
use std::vec::Vec;

#[derive(Debug)]
pub struct CustomSet<T> {
    elements: Vec<T>
}

/// Create an empty set.
impl <T> CustomSet<T> where T: Eq + Clone {
    pub fn new() -> CustomSet<T> {
        CustomSet { elements: Vec::new() }
    }

    pub fn len(&self) -> usize {
        self.elements.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn contains(&self, needle: &T) -> bool {
        for element in self.elements.iter() {
            if needle == element {
                return true;
            }
        }
        false
    }

    pub fn insert(&mut self, element: T) -> bool {
        if !self.contains(&element) {
            self.elements.push(element);
            return true;
        }
        false
    }

    pub fn remove(&mut self, needle: &T) -> bool {
        let mut index: Option<usize> = None;
        for (i, element) in self.iter().enumerate() {
            if needle == element {
                index = Some(i);
                break;
            }
        }
        match index {
            Some(index) => {
                self.elements.remove(index);
                true
            }
            None => false
        }
    }
    
    pub fn difference(&self, other: &CustomSet<T>) -> CustomSet<T> {
        let mut difference: CustomSet<T> = CustomSet::new();
        for element in self.elements.iter() {
            if !other.contains(element) {
                difference.insert(element.clone());
            }
        }
        difference
    }

    pub fn intersection(&self, other: &CustomSet<T>) -> CustomSet<T> {
        let mut intersection: CustomSet<T> = CustomSet::new();
        for element in self.elements.iter() {
            if other.contains(element) {
                intersection.insert(element.clone());
            }
        }
        intersection
    }

    pub fn union(&self, other: &CustomSet<T>) -> CustomSet<T> {
        let mut union = CustomSet::new();
        for element in self.elements.iter().chain(other.elements.iter()) {
            union.insert(element.clone());
        }
        union
    }

    pub fn is_disjoint(&self, other: &CustomSet<T>) -> bool {
        for element in self.elements.iter() {
            if other.contains(element) {
                return false;
            }
        }
        true
    }

    pub fn is_subset(&self, other: &CustomSet<T>) -> bool {
        for element in self.elements.iter() {
            if !other.contains(element) {
                return false;
            }
        }
        true
    }
    
    pub fn is_superset(&self, other: &CustomSet<T>) -> bool {
        other.is_subset(self)
    }

    pub fn clear(&mut self) {
        self.elements.clear();
    }

    pub fn iter(&self) -> Iter<T>{
        Iter { iter: self.elements.iter() }
    }
}

pub struct Iter<'a, T: 'a> {
    iter: slice::Iter<'a, T>
}

impl<'a, T> IntoIterator for &'a CustomSet<T> where T: Eq + Clone {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Iter<'a, T> {
        self.iter()
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        self.iter.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<T> FromIterator<T> for CustomSet<T> where T: Eq + Clone {
    fn from_iter<I: IntoIterator<Item=T>>(iterable: I) -> CustomSet<T> {
        let mut set = CustomSet::new();
        set.elements = Vec::from_iter(iterable);
        set
    }
}
