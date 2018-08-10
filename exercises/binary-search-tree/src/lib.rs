use std::ptr;

#[derive(Debug)]
pub struct TreeNode {
    element: i32,
    pub left: *mut TreeNode,
    pub right: *mut TreeNode,
}

impl PartialEq for TreeNode {
    fn eq(&self, other: &TreeNode) -> bool {
        let elements_are_equal = self.element == other.element;

        let left_nodes_are_equal = if self.left.is_null() {
            other.left.is_null()
        } else if !other.left.is_null() {
            unsafe { *self.left == *other.left }
        } else {
            false
        };

        let right_nodes_are_equal = if self.right.is_null() {
            other.right.is_null()
        } else if !other.right.is_null() {
            unsafe { *self.right == *other.right }
        } else {
            false
        };

        elements_are_equal && left_nodes_are_equal && right_nodes_are_equal
    }
}

pub struct BinarySearchTree {
    root: *mut TreeNode,
}

impl TreeNode {
    pub fn new(element: i32) -> Self {
        TreeNode {
            element,
            left: ptr::null_mut(),
            right: ptr::null_mut(),
        }
    }
}

impl BinarySearchTree {
    pub fn new() -> Self {
        BinarySearchTree {
            root: ptr::null_mut(),
        }
    }

    pub fn get_root(&self) -> *mut TreeNode {
        self.root
    }

    pub fn insert(&mut self, element: i32) {
        let node = TreeNode::new(element);

        self.root = self.insert_node(node, self.root);
    }

    fn insert_node(&self, node: TreeNode, root: *mut TreeNode) -> *mut TreeNode {
        if root.is_null() {
            return Box::into_raw(Box::new(node));
        }

        unsafe {
            if node.element <= (*root).element {
                (*root).left = self.insert_node(node, (*root).left);
            } else {
                (*root).right = self.insert_node(node, (*root).right);
            }
        }

        root
    }
}
