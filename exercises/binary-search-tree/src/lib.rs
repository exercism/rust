use std::marker::PhantomData;

#[derive(Debug, PartialEq)]
pub struct TreeNode<T> {
    phantom_data: PhantomData<T>,
}

pub struct BinarySearchTree<T> {
    phantom_data: PhantomData<T>,
}

impl<T> TreeNode<T> {
    pub fn new(_element: T) -> Self {
        unimplemented!("Construct a new TreeNode from the given element");
    }

    pub fn with_right_node(self, _right_node: TreeNode<T>) -> Self {
        unimplemented!("Add the given node as a right node to the TreeNode struct");
    }

    pub fn with_left_node(self, _left_node: TreeNode<T>) -> Self {
        unimplemented!("Add the given node as a left node to the TreeNode struct",);
    }
}

impl<T> BinarySearchTree<T> {
    pub fn new() -> Self {
        unimplemented!("Costruct a new BinarySearchTree struct.");
    }

    pub fn get_root(&self) -> *mut TreeNode<T> {
        unimplemented!("Return the root of the BinarySearchTree struct.");
    }

    pub fn insert(&mut self, element: i32) {
        unimplemented!(
            "Insert the '{}' element into the BinarySearchTree struct.",
            element
        );
    }
}

impl<T> Into<Vec<T>> for BinarySearchTree<T> {
    fn into(self) -> Vec<T> {
        unimplemented!("Convert the BinarySearchTree struct into Vec.");
    }
}
