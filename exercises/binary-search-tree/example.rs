use std::ptr;

#[derive(Debug)]
pub struct TreeNode<T> {
    element: T,
    left: *mut TreeNode<T>,
    right: *mut TreeNode<T>,
}

impl<T: PartialEq> PartialEq for TreeNode<T> {
    fn eq(&self, other: &TreeNode<T>) -> bool {
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

pub struct BinarySearchTree<T> {
    root: *mut TreeNode<T>,
}

impl<T> TreeNode<T> {
    pub fn new(element: T) -> Self {
        TreeNode {
            element,
            left: ptr::null_mut(),
            right: ptr::null_mut(),
        }
    }

    pub fn with_right_node(mut self, right_node: TreeNode<T>) -> Self {
        self.right = Box::into_raw(Box::new(right_node));

        self
    }

    pub fn with_left_node(mut self, left_node: TreeNode<T>) -> Self {
        self.left = Box::into_raw(Box::new(left_node));

        self
    }
}

impl<T: PartialOrd> BinarySearchTree<T> {
    pub fn new() -> Self {
        BinarySearchTree {
            root: ptr::null_mut(),
        }
    }

    pub fn get_root(&self) -> Option<&mut TreeNode<T>> {
        unsafe { self.root.as_mut() }
    }

    pub fn insert(&mut self, element: T) {
        let node = TreeNode::new(element);

        self.root = self.insert_node(node, self.root);
    }

    fn insert_node(&self, node: TreeNode<T>, root: *mut TreeNode<T>) -> *mut TreeNode<T> {
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

unsafe fn traverse<T>(node: *mut TreeNode<T>, acc_vec: &mut Vec<T>)
where
    T: Clone,
{
    let node = &*node;

    if !node.left.is_null() {
        traverse(node.left, acc_vec);
    }

    acc_vec.push(node.element.clone());

    if !node.right.is_null() {
        traverse(node.right, acc_vec);
    }
}

impl<T: PartialOrd + Clone> Into<Vec<T>> for BinarySearchTree<T> {
    fn into(self) -> Vec<T> {
        let mut result = vec![];

        let root = self.get_root().unwrap();

        unsafe {
            traverse(root, &mut result);
        }

        result
    }
}
