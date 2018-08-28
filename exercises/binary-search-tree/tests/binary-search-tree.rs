extern crate binary_search_tree;

use binary_search_tree::*;

fn process_sort_case(to_sort: &mut [i32]) {
    let mut tree: BinarySearchTree<i32> = BinarySearchTree::new();

    to_sort.iter().for_each(|&element| tree.insert(element));

    to_sort.sort();

    assert_eq!(to_sort.to_vec(), Into::<Vec<i32>>::into(tree));
}

fn process_insertion_case(to_insert: &[i32], expected_root: &TreeNode<i32>) {
    let mut tree: BinarySearchTree<i32> = BinarySearchTree::new();

    to_insert.iter().for_each(|&element| tree.insert(element));

    unsafe {
        assert_eq!(*expected_root, *tree.get_root());
    }
}

#[test]
fn test_empty_tree() {
    let tree: BinarySearchTree<i32> = BinarySearchTree::new();

    assert!(tree.get_root().is_null());
}

#[test]
#[ignore]
/// data is retained
fn test_data_is_retained() {
    let mut tree: BinarySearchTree<i32> = BinarySearchTree::new();

    tree.insert(4);

    unsafe {
        assert_eq!(TreeNode::new(4), *tree.get_root());
    }
}

// insert data at proper node
#[test]
#[ignore]
/// smaller number at left node
fn test_smaller_number_at_left_node() {
    let expected_root = TreeNode::new(4).with_left_node(TreeNode::new(2));

    process_insertion_case(&[4, 2], &expected_root);
}

#[test]
#[ignore]
/// same number at left node
fn test_same_number_at_left_node() {
    let expected_root = TreeNode::new(4).with_left_node(TreeNode::new(4));

    process_insertion_case(&[4, 4], &expected_root);
}

#[test]
#[ignore]
/// greater number at right node
fn test_greater_number_at_right_node() {
    let expected_root = TreeNode::new(4).with_right_node(TreeNode::new(5));

    process_insertion_case(&[4, 5], &expected_root);
}

#[test]
#[ignore]
/// can create complex tree
fn test_can_create_complex_tree() {
    let expected_root = TreeNode::new(4)
        .with_left_node(
            TreeNode::new(2)
                .with_left_node(TreeNode::new(1))
                .with_right_node(TreeNode::new(3)),
        )
        .with_right_node(
            TreeNode::new(6)
                .with_left_node(TreeNode::new(5))
                .with_right_node(TreeNode::new(7)),
        );

    process_insertion_case(&[4, 2, 6, 1, 3, 5, 7], &expected_root);
}

// can sort data
#[test]
#[ignore]
/// can sort single number
fn test_can_sort_single_number() {
    process_sort_case(&mut [2]);
}

#[test]
#[ignore]
/// can sort if second number is smaller than first
fn test_can_sort_if_second_number_is_smaller_than_first() {
    process_sort_case(&mut [2, 1]);
}

#[test]
#[ignore]
/// can sort if second number is same as first
fn test_can_sort_if_second_number_is_same_as_first() {
    process_sort_case(&mut [2, 2]);
}

#[test]
#[ignore]
/// can sort if second number is greater than first
fn test_can_sort_if_second_number_is_greater_than_first() {
    process_sort_case(&mut [2, 3]);
}

#[test]
#[ignore]
/// can sort complex tree
fn test_can_sort_complex_tree() {
    process_sort_case(&mut [2, 1, 3, 6, 7, 5]);
}
