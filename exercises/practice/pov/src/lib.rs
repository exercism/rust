use std::fmt::Debug;

#[derive(Clone, Debug, PartialEq)]
pub struct Tree<T: Clone + Debug + PartialEq> {
    remove_this: std::marker::PhantomData<T>,
}

impl<T: Clone + Debug + PartialEq> Tree<T> {
    pub fn new(label: T) -> Self {
        todo!("Create a new tree with the given {label:?}");
    }

    /// Builder-method for constructing a tree with children
    pub fn with_child(self, child: Self) -> Self {
        todo!("Add {child:?} to the tree and return the tree");
    }

    pub fn label(&self) -> T {
        todo!("Return the tree's label");
    }

    pub fn children(&self) -> impl Iterator<Item = &Self> {
        std::iter::from_fn(|| todo!("Return the tree's children"))
    }

    pub fn pov_from(&self, from: &T) -> Option<Self> {
        todo!("Reparent the tree with the label {from:?} as root");
    }

    pub fn path_to(&self, from: &T, to: &T) -> Option<Vec<T>> {
        todo!("Return the shortest path between {from:?} and {to:?}");
    }
}
