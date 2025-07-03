/// from_pov() tests
/// Reroot a tree so that its root is the specified node.
/// In this way, the tree is presented from the point of view of the specified node.
///
/// If appropriate for your track, you may test that the input tree is not modified.
///
/// Note that when rerooting upon a target node that has both parents and children,
/// it does not matter whether the former parent comes before or after the former children.
/// Please account for this when checking correctness of the resulting trees.
/// One suggested method is to only check two things:
/// 1) The root of the returned tree is the root that was passed in to from_pov.
/// 2) The sorted edge list of the returned tree is the same as the sorted edge list of the expected tree.
mod reroot_a_tree_so_that_its_root_is_the_specified_node {
    use pov::*;

    #[test]
    fn results_in_the_same_tree_if_the_input_tree_is_a_singleton() {
        let input = Tree::new("x".to_string());
        let from = "x".to_string();
        let result = input.pov_from(&from);
        let expected = Some(Tree::new("x".to_string()));
        assert!(crate::test_util::tree_option_eq(result, expected));
    }

    #[test]
    #[ignore]
    fn can_reroot_a_tree_with_a_parent_and_one_sibling() {
        let input = Tree::with_children(
            "parent".to_string(),
            vec![Tree::new("x".to_string()), Tree::new("sibling".to_string())],
        );
        let from = "x".to_string();
        let result = input.pov_from(&from);
        let expected = Some(Tree::with_children(
            "x".to_string(),
            vec![Tree::with_children(
                "parent".to_string(),
                vec![Tree::new("sibling".to_string())],
            )],
        ));
        assert!(crate::test_util::tree_option_eq(result, expected));
    }

    #[test]
    #[ignore]
    fn can_reroot_a_tree_with_a_parent_and_many_siblings() {
        let input = Tree::with_children(
            "parent".to_string(),
            vec![
                Tree::new("a".to_string()),
                Tree::new("x".to_string()),
                Tree::new("b".to_string()),
                Tree::new("c".to_string()),
            ],
        );
        let from = "x".to_string();
        let result = input.pov_from(&from);
        let expected = Some(Tree::with_children(
            "x".to_string(),
            vec![Tree::with_children(
                "parent".to_string(),
                vec![
                    Tree::new("a".to_string()),
                    Tree::new("b".to_string()),
                    Tree::new("c".to_string()),
                ],
            )],
        ));
        assert!(crate::test_util::tree_option_eq(result, expected));
    }

    #[test]
    #[ignore]
    fn can_reroot_a_tree_with_new_root_deeply_nested_in_tree() {
        let input = Tree::with_children(
            "level-0".to_string(),
            vec![Tree::with_children(
                "level-1".to_string(),
                vec![Tree::with_children(
                    "level-2".to_string(),
                    vec![Tree::with_children(
                        "level-3".to_string(),
                        vec![Tree::new("x".to_string())],
                    )],
                )],
            )],
        );
        let from = "x".to_string();
        let result = input.pov_from(&from);
        let expected = Some(Tree::with_children(
            "x".to_string(),
            vec![Tree::with_children(
                "level-3".to_string(),
                vec![Tree::with_children(
                    "level-2".to_string(),
                    vec![Tree::with_children(
                        "level-1".to_string(),
                        vec![Tree::new("level-0".to_string())],
                    )],
                )],
            )],
        ));
        assert!(crate::test_util::tree_option_eq(result, expected));
    }

    #[test]
    #[ignore]
    fn moves_children_of_the_new_root_to_same_level_as_former_parent() {
        let input = Tree::with_children(
            "parent".to_string(),
            vec![Tree::with_children(
                "x".to_string(),
                vec![
                    Tree::new("kid-0".to_string()),
                    Tree::new("kid-1".to_string()),
                ],
            )],
        );
        let from = "x".to_string();
        let result = input.pov_from(&from);
        let expected = Some(Tree::with_children(
            "x".to_string(),
            vec![
                Tree::new("kid-0".to_string()),
                Tree::new("kid-1".to_string()),
                Tree::new("parent".to_string()),
            ],
        ));
        assert!(crate::test_util::tree_option_eq(result, expected));
    }

    #[test]
    #[ignore]
    fn can_reroot_a_complex_tree_with_cousins() {
        let input = Tree::with_children(
            "grandparent".to_string(),
            vec![
                Tree::with_children(
                    "parent".to_string(),
                    vec![
                        Tree::with_children(
                            "x".to_string(),
                            vec![
                                Tree::new("kid-0".to_string()),
                                Tree::new("kid-1".to_string()),
                            ],
                        ),
                        Tree::new("sibling-0".to_string()),
                        Tree::new("sibling-1".to_string()),
                    ],
                ),
                Tree::with_children(
                    "uncle".to_string(),
                    vec![
                        Tree::new("cousin-0".to_string()),
                        Tree::new("cousin-1".to_string()),
                    ],
                ),
            ],
        );
        let from = "x".to_string();
        let result = input.pov_from(&from);
        let expected = Some(Tree::with_children(
            "x".to_string(),
            vec![
                Tree::new("kid-1".to_string()),
                Tree::new("kid-0".to_string()),
                Tree::with_children(
                    "parent".to_string(),
                    vec![
                        Tree::new("sibling-0".to_string()),
                        Tree::new("sibling-1".to_string()),
                        Tree::with_children(
                            "grandparent".to_string(),
                            vec![Tree::with_children(
                                "uncle".to_string(),
                                vec![
                                    Tree::new("cousin-0".to_string()),
                                    Tree::new("cousin-1".to_string()),
                                ],
                            )],
                        ),
                    ],
                ),
            ],
        ));
        assert!(crate::test_util::tree_option_eq(result, expected));
    }

    #[test]
    #[ignore]
    fn errors_if_target_does_not_exist_in_a_singleton_tree() {
        let input = Tree::new("x".to_string());
        let from = "nonexistent".to_string();
        let result = input.pov_from(&from);
        let expected: Option<Tree<String>> = None;
        assert!(crate::test_util::tree_option_eq(result, expected));
    }

    #[test]
    #[ignore]
    fn errors_if_target_does_not_exist_in_a_large_tree() {
        let input = Tree::with_children(
            "parent".to_string(),
            vec![
                Tree::with_children(
                    "x".to_string(),
                    vec![
                        Tree::new("kid-0".to_string()),
                        Tree::new("kid-1".to_string()),
                    ],
                ),
                Tree::new("sibling-0".to_string()),
                Tree::new("sibling-1".to_string()),
            ],
        );
        let from = "nonexistent".to_string();
        let result = input.pov_from(&from);
        let expected: Option<Tree<String>> = None;
        assert!(crate::test_util::tree_option_eq(result, expected));
    }
}

/// path_to() tests
/// Given two nodes, find the path between them
/// A typical implementation would first reroot the tree on one of the two nodes.
///
/// If appropriate for your track, you may test that the input tree is not modified.
mod given_two_nodes_find_the_path_between_them {
    use pov::*;

    #[test]
    #[ignore]
    fn can_find_path_to_parent() {
        let input = Tree::with_children(
            "parent".to_string(),
            vec![Tree::new("x".to_string()), Tree::new("sibling".to_string())],
        );
        let from = "x".to_string();
        let to = "parent".to_string();
        let result = input.path_to(&from, &to);
        let expected = Some(vec!["x".to_string(), "parent".to_string()]);
        assert_eq!(result, expected);
    }

    #[test]
    #[ignore]
    fn can_find_path_to_sibling() {
        let input = Tree::with_children(
            "parent".to_string(),
            vec![
                Tree::new("a".to_string()),
                Tree::new("x".to_string()),
                Tree::new("b".to_string()),
                Tree::new("c".to_string()),
            ],
        );
        let from = "x".to_string();
        let to = "b".to_string();
        let result = input.path_to(&from, &to);
        let expected = Some(vec!["x".to_string(), "parent".to_string(), "b".to_string()]);
        assert_eq!(result, expected);
    }

    #[test]
    #[ignore]
    fn can_find_path_to_cousin() {
        let input = Tree::with_children(
            "grandparent".to_string(),
            vec![
                Tree::with_children(
                    "parent".to_string(),
                    vec![
                        Tree::with_children(
                            "x".to_string(),
                            vec![
                                Tree::new("kid-0".to_string()),
                                Tree::new("kid-1".to_string()),
                            ],
                        ),
                        Tree::new("sibling-0".to_string()),
                        Tree::new("sibling-1".to_string()),
                    ],
                ),
                Tree::with_children(
                    "uncle".to_string(),
                    vec![
                        Tree::new("cousin-0".to_string()),
                        Tree::new("cousin-1".to_string()),
                    ],
                ),
            ],
        );
        let from = "x".to_string();
        let to = "cousin-1".to_string();
        let result = input.path_to(&from, &to);
        let expected = Some(vec![
            "x".to_string(),
            "parent".to_string(),
            "grandparent".to_string(),
            "uncle".to_string(),
            "cousin-1".to_string(),
        ]);
        assert_eq!(result, expected);
    }

    #[test]
    #[ignore]
    fn can_find_path_not_involving_root() {
        let input = Tree::with_children(
            "grandparent".to_string(),
            vec![Tree::with_children(
                "parent".to_string(),
                vec![
                    Tree::new("x".to_string()),
                    Tree::new("sibling-0".to_string()),
                    Tree::new("sibling-1".to_string()),
                ],
            )],
        );
        let from = "x".to_string();
        let to = "sibling-1".to_string();
        let result = input.path_to(&from, &to);
        let expected = Some(vec![
            "x".to_string(),
            "parent".to_string(),
            "sibling-1".to_string(),
        ]);
        assert_eq!(result, expected);
    }

    #[test]
    #[ignore]
    fn can_find_path_from_nodes_other_than_x() {
        let input = Tree::with_children(
            "parent".to_string(),
            vec![
                Tree::new("a".to_string()),
                Tree::new("x".to_string()),
                Tree::new("b".to_string()),
                Tree::new("c".to_string()),
            ],
        );
        let from = "a".to_string();
        let to = "c".to_string();
        let result = input.path_to(&from, &to);
        let expected = Some(vec!["a".to_string(), "parent".to_string(), "c".to_string()]);
        assert_eq!(result, expected);
    }

    #[test]
    #[ignore]
    fn errors_if_destination_does_not_exist() {
        let input = Tree::with_children(
            "parent".to_string(),
            vec![
                Tree::with_children(
                    "x".to_string(),
                    vec![
                        Tree::new("kid-0".to_string()),
                        Tree::new("kid-1".to_string()),
                    ],
                ),
                Tree::new("sibling-0".to_string()),
                Tree::new("sibling-1".to_string()),
            ],
        );
        let from = "x".to_string();
        let to = "nonexistent".to_string();
        let result = input.path_to(&from, &to);
        let expected: Option<Vec<String>> = None;
        assert_eq!(result, expected);
    }

    #[test]
    #[ignore]
    fn errors_if_source_does_not_exist() {
        let input = Tree::with_children(
            "parent".to_string(),
            vec![
                Tree::with_children(
                    "x".to_string(),
                    vec![
                        Tree::new("kid-0".to_string()),
                        Tree::new("kid-1".to_string()),
                    ],
                ),
                Tree::new("sibling-0".to_string()),
                Tree::new("sibling-1".to_string()),
            ],
        );
        let from = "nonexistent".to_string();
        let to = "x".to_string();
        let result = input.path_to(&from, &to);
        let expected: Option<Vec<String>> = None;
        assert_eq!(result, expected);
    }
}

mod test_util {
    use pov::*;

    pub fn tree_option_eq(lhs: Option<Tree<String>>, rhs: Option<Tree<String>>) -> bool {
        match (lhs, rhs) {
            (None, None) => true,
            (Some(l_inner), Some(r_inner)) => tree_eq(&l_inner, &r_inner),
            _ => false,
        }
    }

    pub fn tree_eq(lhs: &Tree<String>, rhs: &Tree<String>) -> bool {
        let (l_label, r_label) = (lhs.get_label(), rhs.get_label());
        let (mut l_children, mut r_children) = (lhs.get_children(), rhs.get_children());
        if l_label == r_label && l_children.len() == r_children.len() {
            if l_children.len() == 0 {
                return true;
            }
            let key_fn = |child: &&Tree<String>| child.get_label();
            l_children.sort_unstable_by_key(key_fn);
            r_children.sort_unstable_by_key(key_fn);
            return l_children
                .iter()
                .zip(r_children.iter())
                .all(|(&lc, &rc)| tree_eq(lc, rc));
        }
        false
    }
}
