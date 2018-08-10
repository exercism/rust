extern crate binary_search_tree;

use binary_search_tree::*;

#[test]
fn test_empty_tree() {
    let tree = BinarySearchTree::new();

    assert!(tree.get_root().is_null());
}

/*
fn process_sorteddata_case<I, O>(input: I, expected: O) {
    // typical implementation:
    // assert_eq!(
    //     student_sortedData_func(input),
    //     expected
    // )
    unimplemented!()
}

fn process_data_case<I, O>(input: I, expected: O) {
    // typical implementation:
    // assert_eq!(
    //     student_data_func(input),
    //     expected
    // )
    unimplemented!()
}

#[test]
/// data is retained
fn test_data_is_retained() {
    process_data_case(
        {
            let mut hm = ::std::collections::HashMap::new();
            hm.insert("treeData", vec!["4"]);
            hm
        },
        {
            let mut hm = ::std::collections::HashMap::new();
            hm.insert("data", "4");
            hm.insert("left", None);
            hm.insert("right", None);
            hm
        },
    );
}

// insert data at proper node

#[test]
#[ignore]
/// smaller number at left node
fn test_smaller_number_at_left_node() {
    process_data_case(
        {
            let mut hm = ::std::collections::HashMap::new();
            hm.insert("treeData", vec!["4", "2"]);
            hm
        },
        {
            let mut hm = ::std::collections::HashMap::new();
            hm.insert("data", "4");
            hm.insert("left", {
                let mut hm = ::std::collections::HashMap::new();
                hm.insert("data", "2");
                hm.insert("left", None);
                hm.insert("right", None);
                hm
            });
            hm.insert("right", None);
            hm
        },
    );
}

#[test]
#[ignore]
/// same number at left node
fn test_same_number_at_left_node() {
    process_data_case(
        {
            let mut hm = ::std::collections::HashMap::new();
            hm.insert("treeData", vec!["4", "4"]);
            hm
        },
        {
            let mut hm = ::std::collections::HashMap::new();
            hm.insert("data", "4");
            hm.insert("left", {
                let mut hm = ::std::collections::HashMap::new();
                hm.insert("data", "4");
                hm.insert("left", None);
                hm.insert("right", None);
                hm
            });
            hm.insert("right", None);
            hm
        },
    );
}

#[test]
#[ignore]
/// greater number at right node
fn test_greater_number_at_right_node() {
    process_data_case(
        {
            let mut hm = ::std::collections::HashMap::new();
            hm.insert("treeData", vec!["4", "5"]);
            hm
        },
        {
            let mut hm = ::std::collections::HashMap::new();
            hm.insert("data", "4");
            hm.insert("left", None);
            hm.insert("right", {
                let mut hm = ::std::collections::HashMap::new();
                hm.insert("data", "5");
                hm.insert("left", None);
                hm.insert("right", None);
                hm
            });
            hm
        },
    );
}

#[test]
#[ignore]
/// can create complex tree
fn test_can_create_complex_tree() {
    process_data_case(
        {
            let mut hm = ::std::collections::HashMap::new();
            hm.insert("treeData", vec!["4", "2", "6", "1", "3", "5", "7"]);
            hm
        },
        {
            let mut hm = ::std::collections::HashMap::new();
            hm.insert("data", "4");
            hm.insert("left", {
                let mut hm = ::std::collections::HashMap::new();
                hm.insert("data", "2");
                hm.insert("left", {
                    let mut hm = ::std::collections::HashMap::new();
                    hm.insert("data", "1");
                    hm.insert("left", None);
                    hm.insert("right", None);
                    hm
                });
                hm.insert("right", {
                    let mut hm = ::std::collections::HashMap::new();
                    hm.insert("data", "3");
                    hm.insert("left", None);
                    hm.insert("right", None);
                    hm
                });
                hm
            });
            hm.insert("right", {
                let mut hm = ::std::collections::HashMap::new();
                hm.insert("data", "6");
                hm.insert("left", {
                    let mut hm = ::std::collections::HashMap::new();
                    hm.insert("data", "5");
                    hm.insert("left", None);
                    hm.insert("right", None);
                    hm
                });
                hm.insert("right", {
                    let mut hm = ::std::collections::HashMap::new();
                    hm.insert("data", "7");
                    hm.insert("left", None);
                    hm.insert("right", None);
                    hm
                });
                hm
            });
            hm
        },
    );
}

// can sort data

#[test]
#[ignore]
/// can sort single number
fn test_can_sort_single_number() {
    process_sorteddata_case(
        {
            let mut hm = ::std::collections::HashMap::new();
            hm.insert("treeData", vec!["2"]);
            hm
        },
        vec!["2"],
    );
}

#[test]
#[ignore]
/// can sort if second number is smaller than first
fn test_can_sort_if_second_number_is_smaller_than_first() {
    process_sorteddata_case(
        {
            let mut hm = ::std::collections::HashMap::new();
            hm.insert("treeData", vec!["2", "1"]);
            hm
        },
        vec!["1", "2"],
    );
}

#[test]
#[ignore]
/// can sort if second number is same as first
fn test_can_sort_if_second_number_is_same_as_first() {
    process_sorteddata_case(
        {
            let mut hm = ::std::collections::HashMap::new();
            hm.insert("treeData", vec!["2", "2"]);
            hm
        },
        vec!["2", "2"],
    );
}

#[test]
#[ignore]
/// can sort if second number is greater than first
fn test_can_sort_if_second_number_is_greater_than_first() {
    process_sorteddata_case(
        {
            let mut hm = ::std::collections::HashMap::new();
            hm.insert("treeData", vec!["2", "3"]);
            hm
        },
        vec!["2", "3"],
    );
}

#[test]
#[ignore]
/// can sort complex tree
fn test_can_sort_complex_tree() {
    process_sorteddata_case(
        {
            let mut hm = ::std::collections::HashMap::new();
            hm.insert("treeData", vec!["2", "1", "3", "6", "7", "5"]);
            hm
        },
        vec!["1", "2", "3", "5", "6", "7"],
    );
}*/
