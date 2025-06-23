use std::collections::HashSet;
use std::hash::Hash;

#[derive(Debug, PartialEq, Eq)]
pub struct CustomSet<T: Eq + Hash> {
    // We fake using T here, so the compiler does not complain that
    // "parameter `T` is never used". Delete when no longer needed.
    elements: HashSet<T>
}

impl<T: Eq + Hash + Clone> CustomSet<T> {
    pub fn new(_input: &[T]) -> Self {
        Self {
            elements: _input.iter().cloned().collect()
        }
    }

    pub fn contains(&self, _element: &T) -> bool {
        self.elements.contains(_element)
    }

    pub fn add(&mut self, _element: T) {
        self.elements.insert(_element);
    }

    pub fn is_subset(&self, _other: &Self) -> bool {
        self.elements.is_subset(&_other.elements)
    }

    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }

    pub fn is_disjoint(&self, _other: &Self) -> bool {
        self.elements.is_disjoint(&_other.elements)
    }

    #[must_use]
    pub fn intersection(&self, _other: &Self) -> Self {
        Self {
            elements: self.elements.intersection(&_other.elements).cloned().collect()
        }
    }

    #[must_use]
    pub fn difference(&self, _other: &Self) -> Self {
        Self {
            elements: self.elements.difference(&_other.elements).cloned().collect()
        }
    }

    #[must_use]
    pub fn union(&self, _other: &Self) -> Self {
        Self { elements: self.elements.union(&_other.elements).cloned().collect() }
    }
}
