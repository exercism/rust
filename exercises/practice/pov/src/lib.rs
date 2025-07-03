use std::fmt::Debug;

#[derive(Clone, Debug, PartialEq)]
pub struct Tree<T: Clone + Debug + PartialEq> {
    _remove_this: std::marker::PhantomData<T>,
}

impl<T: Clone + Debug + PartialEq> Tree<T> {
    pub fn new(label: T) -> Self {
        todo!("Implement a function that creates a new Tree with given {label:?}");
    }

    pub fn with_children(label: T, children: Vec<Self>) -> Self {
        todo!("Implement a function that creates a new Tree with given {label:?} and {children:?}");
    }

    pub fn get_label(&self) -> T {
        todo!("Implement getter for label.");
    }

    pub fn get_children(&self) -> Vec<&Self> {
        todo!("Implement getter for children.");
    }

    pub fn pov_from(&self, from: &T) -> Option<Self> {
        todo!("Implement a function that reparents Tree with {from:?} as root.");
    }

    pub fn path_to(&self, from: &T, to: &T) -> Option<Vec<T>> {
        todo!(
            "Implement a function that returns the list of labels in the shortest path from {from:?} to {to:?}"
        );
    }
}
