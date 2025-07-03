/// Reroot a tree so that its root is the specified node.
mod frompov {
    use pov::*;

    #[test]
    fn results_in_the_same_tree_if_the_input_tree_is_a_singleton() {
        let input = Tree::new("x".to_string());
        let from = "x".to_string();
        let result = input.pov_from(&from);
        let expected = Some(Tree::new("x".to_string()));
        assert_eq!(
            result.map(|t| crate::test_util::tree_to_sorted(&t)),
            expected.map(|t| crate::test_util::tree_to_sorted(&t))
        );
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
        assert_eq!(
            result.map(|t| crate::test_util::tree_to_sorted(&t)),
            expected.map(|t| crate::test_util::tree_to_sorted(&t))
        );
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
        assert_eq!(
            result.map(|t| crate::test_util::tree_to_sorted(&t)),
            expected.map(|t| crate::test_util::tree_to_sorted(&t))
        );
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
        assert_eq!(
            result.map(|t| crate::test_util::tree_to_sorted(&t)),
            expected.map(|t| crate::test_util::tree_to_sorted(&t))
        );
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
        assert_eq!(
            result.map(|t| crate::test_util::tree_to_sorted(&t)),
            expected.map(|t| crate::test_util::tree_to_sorted(&t))
        );
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
        assert_eq!(
            result.map(|t| crate::test_util::tree_to_sorted(&t)),
            expected.map(|t| crate::test_util::tree_to_sorted(&t))
        );
    }

    #[test]
    #[ignore]
    fn errors_if_target_does_not_exist_in_a_singleton_tree() {
        let input = Tree::new("x".to_string());
        let from = "nonexistent".to_string();
        let result = input.pov_from(&from);
        let expected: Option<Tree<String>> = None;
        assert_eq!(
            result.map(|t| crate::test_util::tree_to_sorted(&t)),
            expected.map(|t| crate::test_util::tree_to_sorted(&t))
        );
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
        assert_eq!(
            result.map(|t| crate::test_util::tree_to_sorted(&t)),
            expected.map(|t| crate::test_util::tree_to_sorted(&t))
        );
    }
}

/// Given two nodes, find the path between them
mod pathto {
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
    pub fn tree_to_sorted<T: Ord + Clone + std::fmt::Debug>(tree: &Tree<T>) -> Tree<T> {
        let mut children = tree.get_children();
        children.sort_unstable_by_key(|child| child.get_label());
        Tree::with_children(tree.get_label(), children.into_iter().cloned().collect())
    }
}
