use std::fmt::Debug;
use std::marker::PhantomData;

#[derive(Debug, PartialEq)]
pub struct CustomSet<T> {
    // This field is here to make the template compile and not to
    // complain about unused type parameter 'T'. Once you start
    // solving the exercise, delete this field and the 'std::marker::PhantomData'
    // import.
    phantom: PhantomData<T>,
}

impl<T: Debug> CustomSet<T> {
    pub fn new(input: &[T]) -> Self {
        unimplemented!(
            "From the given input '{:?}' construct a new CustomSet struct.",
            input
        );
    }

    pub fn contains(&self, element: &T) -> bool {
        unimplemented!(
            "Determine if the '{:?}' element is present in the CustomSet struct.",
            element
        );
    }

    pub fn add(&mut self, element: T) {
        unimplemented!("Add the '{:?}' element to the CustomSet struct.", element);
    }

    pub fn is_subset(&self, other: &Self) -> bool {
        unimplemented!(
            "Determine if the CustomSet struct is a subset of the other '{:?}' struct.",
            other
        );
    }

    pub fn is_empty(&self) -> bool {
        unimplemented!("Determine if the CustomSet struct is empty.");
    }

    pub fn is_disjoint(&self, other: &Self) -> bool {
        unimplemented!(
            "Determine if the CustomSet struct and the other struct '{:?}' are disjoint.",
            other
        );
    }

    pub fn intersection(&self, other: &Self) -> Self {
        unimplemented!("Construct a new CustomSet struct that is an intersection between current struct and the other struct '{:?}'.", other);
    }

    pub fn difference(&self, other: &Self) -> Self {
        unimplemented!("Construct a new CustomSet struct that is a difference between current struct and the other struct '{:?}'.", other);
    }

    pub fn union(&self, other: &Self) -> Self {
        unimplemented!("Construct a new CustomSet struct that is an union between current struct and the other struct '{:?}'.", other);
    }
}
