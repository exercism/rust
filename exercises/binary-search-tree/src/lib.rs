use std::ptr;

pub struct TreeNode {
    element: i32,
    left: *mut TreeNode,
    right: *mut TreeNode,
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
            if node.element < (*root).element {
                (*root).left = self.insert_node(node, (*root).left);
            } else {
                (*root).right = self.insert_node(node, (*root).right);
            }
        }

        root
    }
}
