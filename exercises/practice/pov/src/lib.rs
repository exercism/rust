use std::fmt::Debug;

#[derive(Clone, Debug, PartialEq)]
pub struct Tree<T: Clone + Debug + PartialEq> {
    remove_this: std::marker::PhantomData<T>,
}

impl<T: Clone + Debug + PartialEq> Tree<T> {
    pub fn new(label: T) -> Self {
        todo!("Create a new tree with the given {label:?}");
    }

    pub fn with_children(label: T, children: Vec<Self>) -> Self {
        todo!("Creaet a new tree with the given {label:?} and {children:?}");
    }

    pub fn label(&self) -> T {
        todo!("Return the tree's label");
    }

    pub fn children(&self) -> Vec<&Self> {
        todo!("Return the tree's children");
    }

    pub fn pov_from(&self, from: &T) -> Option<Self> {
        todo!("Reparent the tree with the label {from:?} as root");
    }

    pub fn path_to(&self, from: &T, to: &T) -> Option<Vec<T>> {
        todo!("Return the shortest path between {from:?} and {to:?}");
    }
}
