use std::fmt::Debug;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Tree<T: Debug + Ord> {
    remove_this: std::marker::PhantomData<T>,
}

impl<T: Debug + Ord> Tree<T> {
    pub fn new(label: T) -> Self {
        todo!("Create a new tree with the given {label:?}");
    }

    /// Builder-method for constructing a tree with children
    pub fn with_child(self, child: Self) -> Self {
        todo!("Add {child:?} to the tree and return the tree");
    }

    pub fn pov_from(&mut self, from: &T) -> bool {
        todo!("Reparent the tree with the label {from:?} as root");
    }

    pub fn path_between<'a>(&'a mut self, from: &'a T, to: &'a T) -> Option<Vec<&'a T>> {
        todo!("Return the shortest path between {from:?} and {to:?}");
    }
}
