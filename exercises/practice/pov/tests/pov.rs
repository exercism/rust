/// Reroot a tree so that its root is the specified node.
mod from_pov {
    use super::test_util::tree_to_sorted;
    use pov::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn results_in_the_same_tree_if_the_input_tree_is_a_singleton() {
        let input = Tree::new("x".to_string());
        let from = "x".to_string();
        let result = input.pov_from(&from);
        let expected = Some(Tree::new("x".to_string()));
        assert_eq!(tree_to_sorted(result), tree_to_sorted(expected));
    }

    #[test]
    #[ignore]
    fn can_reroot_a_tree_with_a_parent_and_one_sibling() {
        let input = Tree::new("parent".to_string())
            .with_child(Tree::new("x".to_string()))
            .with_child(Tree::new("sibling".to_string()));
        let from = "x".to_string();
        let result = input.pov_from(&from);
        let expected = Some(Tree::new("x".to_string()).with_child(
            Tree::new("parent".to_string()).with_child(Tree::new("sibling".to_string())),
        ));
        assert_eq!(tree_to_sorted(result), tree_to_sorted(expected));
    }

    #[test]
    #[ignore]
    fn can_reroot_a_tree_with_a_parent_and_many_siblings() {
        let input = Tree::new("parent".to_string())
            .with_child(Tree::new("a".to_string()))
            .with_child(Tree::new("x".to_string()))
            .with_child(Tree::new("b".to_string()))
            .with_child(Tree::new("c".to_string()));
        let from = "x".to_string();
        let result = input.pov_from(&from);
        let expected = Some(
            Tree::new("x".to_string()).with_child(
                Tree::new("parent".to_string())
                    .with_child(Tree::new("a".to_string()))
                    .with_child(Tree::new("b".to_string()))
                    .with_child(Tree::new("c".to_string())),
            ),
        );
        assert_eq!(tree_to_sorted(result), tree_to_sorted(expected));
    }

    #[test]
    #[ignore]
    fn can_reroot_a_tree_with_new_root_deeply_nested_in_tree() {
        let input = Tree::new("level-0".to_string()).with_child(
            Tree::new("level-1".to_string()).with_child(
                Tree::new("level-2".to_string()).with_child(
                    Tree::new("level-3".to_string()).with_child(Tree::new("x".to_string())),
                ),
            ),
        );
        let from = "x".to_string();
        let result = input.pov_from(&from);
        let expected = Some(Tree::new("x".to_string()).with_child(
            Tree::new("level-3".to_string()).with_child(
                Tree::new("level-2".to_string()).with_child(
                    Tree::new("level-1".to_string()).with_child(Tree::new("level-0".to_string())),
                ),
            ),
        ));
        assert_eq!(tree_to_sorted(result), tree_to_sorted(expected));
    }

    #[test]
    #[ignore]
    fn moves_children_of_the_new_root_to_same_level_as_former_parent() {
        let input = Tree::new("parent".to_string()).with_child(
            Tree::new("x".to_string())
                .with_child(Tree::new("kid-0".to_string()))
                .with_child(Tree::new("kid-1".to_string())),
        );
        let from = "x".to_string();
        let result = input.pov_from(&from);
        let expected = Some(
            Tree::new("x".to_string())
                .with_child(Tree::new("kid-0".to_string()))
                .with_child(Tree::new("kid-1".to_string()))
                .with_child(Tree::new("parent".to_string())),
        );
        assert_eq!(tree_to_sorted(result), tree_to_sorted(expected));
    }

    #[test]
    #[ignore]
    fn can_reroot_a_complex_tree_with_cousins() {
        let input = Tree::new("grandparent".to_string())
            .with_child(
                Tree::new("parent".to_string())
                    .with_child(
                        Tree::new("x".to_string())
                            .with_child(Tree::new("kid-0".to_string()))
                            .with_child(Tree::new("kid-1".to_string())),
                    )
                    .with_child(Tree::new("sibling-0".to_string()))
                    .with_child(Tree::new("sibling-1".to_string())),
            )
            .with_child(
                Tree::new("uncle".to_string())
                    .with_child(Tree::new("cousin-0".to_string()))
                    .with_child(Tree::new("cousin-1".to_string())),
            );
        let from = "x".to_string();
        let result = input.pov_from(&from);
        let expected = Some(
            Tree::new("x".to_string())
                .with_child(Tree::new("kid-1".to_string()))
                .with_child(Tree::new("kid-0".to_string()))
                .with_child(
                    Tree::new("parent".to_string())
                        .with_child(Tree::new("sibling-0".to_string()))
                        .with_child(Tree::new("sibling-1".to_string()))
                        .with_child(
                            Tree::new("grandparent".to_string()).with_child(
                                Tree::new("uncle".to_string())
                                    .with_child(Tree::new("cousin-0".to_string()))
                                    .with_child(Tree::new("cousin-1".to_string())),
                            ),
                        ),
                ),
        );
        assert_eq!(tree_to_sorted(result), tree_to_sorted(expected));
    }

    #[test]
    #[ignore]
    fn errors_if_target_does_not_exist_in_a_singleton_tree() {
        let input = Tree::new("x".to_string());
        let from = "nonexistent".to_string();
        let result = input.pov_from(&from);
        let expected: Option<Tree<String>> = None;
        assert_eq!(tree_to_sorted(result), tree_to_sorted(expected));
    }

    #[test]
    #[ignore]
    fn errors_if_target_does_not_exist_in_a_large_tree() {
        let input = Tree::new("parent".to_string())
            .with_child(
                Tree::new("x".to_string())
                    .with_child(Tree::new("kid-0".to_string()))
                    .with_child(Tree::new("kid-1".to_string())),
            )
            .with_child(Tree::new("sibling-0".to_string()))
            .with_child(Tree::new("sibling-1".to_string()));
        let from = "nonexistent".to_string();
        let result = input.pov_from(&from);
        let expected: Option<Tree<String>> = None;
        assert_eq!(tree_to_sorted(result), tree_to_sorted(expected));
    }
}

/// Given two nodes, find the path between them
mod path_to {
    use pov::*;
    use pretty_assertions::assert_eq;

    #[test]
    #[ignore]
    fn can_find_path_to_parent() {
        let input = Tree::new("parent".to_string())
            .with_child(Tree::new("x".to_string()))
            .with_child(Tree::new("sibling".to_string()));
        let from = "x".to_string();
        let to = "parent".to_string();
        let result = input.path_to(&from, &to);
        let expected = Some(vec!["x".to_string(), "parent".to_string()]);
        assert_eq!(result, expected);
    }

    #[test]
    #[ignore]
    fn can_find_path_to_sibling() {
        let input = Tree::new("parent".to_string())
            .with_child(Tree::new("a".to_string()))
            .with_child(Tree::new("x".to_string()))
            .with_child(Tree::new("b".to_string()))
            .with_child(Tree::new("c".to_string()));
        let from = "x".to_string();
        let to = "b".to_string();
        let result = input.path_to(&from, &to);
        let expected = Some(vec!["x".to_string(), "parent".to_string(), "b".to_string()]);
        assert_eq!(result, expected);
    }

    #[test]
    #[ignore]
    fn can_find_path_to_cousin() {
        let input = Tree::new("grandparent".to_string())
            .with_child(
                Tree::new("parent".to_string())
                    .with_child(
                        Tree::new("x".to_string())
                            .with_child(Tree::new("kid-0".to_string()))
                            .with_child(Tree::new("kid-1".to_string())),
                    )
                    .with_child(Tree::new("sibling-0".to_string()))
                    .with_child(Tree::new("sibling-1".to_string())),
            )
            .with_child(
                Tree::new("uncle".to_string())
                    .with_child(Tree::new("cousin-0".to_string()))
                    .with_child(Tree::new("cousin-1".to_string())),
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
        let input = Tree::new("grandparent".to_string()).with_child(
            Tree::new("parent".to_string())
                .with_child(Tree::new("x".to_string()))
                .with_child(Tree::new("sibling-0".to_string()))
                .with_child(Tree::new("sibling-1".to_string())),
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
        let input = Tree::new("parent".to_string())
            .with_child(Tree::new("a".to_string()))
            .with_child(Tree::new("x".to_string()))
            .with_child(Tree::new("b".to_string()))
            .with_child(Tree::new("c".to_string()));
        let from = "a".to_string();
        let to = "c".to_string();
        let result = input.path_to(&from, &to);
        let expected = Some(vec!["a".to_string(), "parent".to_string(), "c".to_string()]);
        assert_eq!(result, expected);
    }

    #[test]
    #[ignore]
    fn errors_if_destination_does_not_exist() {
        let input = Tree::new("parent".to_string())
            .with_child(
                Tree::new("x".to_string())
                    .with_child(Tree::new("kid-0".to_string()))
                    .with_child(Tree::new("kid-1".to_string())),
            )
            .with_child(Tree::new("sibling-0".to_string()))
            .with_child(Tree::new("sibling-1".to_string()));
        let from = "x".to_string();
        let to = "nonexistent".to_string();
        let result = input.path_to(&from, &to);
        let expected: Option<Vec<String>> = None;
        assert_eq!(result, expected);
    }

    #[test]
    #[ignore]
    fn errors_if_source_does_not_exist() {
        let input = Tree::new("parent".to_string())
            .with_child(
                Tree::new("x".to_string())
                    .with_child(Tree::new("kid-0".to_string()))
                    .with_child(Tree::new("kid-1".to_string())),
            )
            .with_child(Tree::new("sibling-0".to_string()))
            .with_child(Tree::new("sibling-1".to_string()));
        let from = "nonexistent".to_string();
        let to = "x".to_string();
        let result = input.path_to(&from, &to);
        let expected: Option<Vec<String>> = None;
        assert_eq!(result, expected);
    }
}

mod test_util {
    use pov::*;
    use std::fmt::Debug;

    pub fn tree_to_sorted<T: Ord + Clone + Debug>(tree_opt: Option<Tree<T>>) -> Option<Tree<T>> {
        tree_opt.map(sorter)
    }

    fn sorter<T: Ord + Clone + Debug>(tree: Tree<T>) -> Tree<T> {
        let mut children = tree.children().collect::<Vec<_>>();
        children.sort_unstable_by_key(|child| child.label());
        let mut tree = Tree::new(tree.label());
        for child in children {
            tree = tree.with_child(sorter(child.clone()));
        }
        tree
    }
}
