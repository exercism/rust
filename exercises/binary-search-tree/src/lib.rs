#[derive(Debug, PartialEq)]
pub struct TreeNode;

pub struct BinarySearchTree;

impl TreeNode {
    pub fn new(element: i32) -> Self {
        unimplemented!("Construct a new TreeNode from the '{}' element", element);
    }

    pub fn with_right_node(self, right_node: TreeNode) -> Self {
        unimplemented!(
            "Add the {:?} node as a right node to the TreeNode struct",
            right_node
        );
    }

    pub fn with_left_node(self, left_node: TreeNode) -> Self {
        unimplemented!(
            "Add the {:?} node as a left node to the TreeNode struct",
            left_node
        );
    }
}

impl BinarySearchTree {
    pub fn new() -> Self {
        unimplemented!("Costruct a new BinarySearchTree struct.");
    }

    pub fn get_root(&self) -> *mut TreeNode {
        unimplemented!("Return the root of the BinarySearchTree struct.");
    }

    pub fn insert(&mut self, element: i32) {
        unimplemented!(
            "Insert the '{}' element into the BinarySearchTree struct.",
            element
        );
    }
}

impl Into<Vec<i32>> for BinarySearchTree {
    fn into(self) -> Vec<i32> {
        unimplemented!("Convert the BinarySearchTree struct into Vec.");
    }
}
