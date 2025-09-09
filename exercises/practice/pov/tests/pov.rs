/// Reroot a tree so that its root is the specified node.
mod from_pov {
    use pov::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn results_in_the_same_tree_if_the_input_tree_is_a_singleton() {
        let mut tree = Tree::new("x");
        let from = "x";
        assert!(tree.pov_from(&from));
        let expected = Tree::new("x");
        assert_eq!(tree, expected);
    }

    #[test]
    #[ignore]
    fn can_reroot_a_tree_with_a_parent_and_one_sibling() {
        let mut tree = Tree::new("parent")
            .with_child(Tree::new("x"))
            .with_child(Tree::new("sibling"));
        let from = "x";
        assert!(tree.pov_from(&from));
        let expected =
            Tree::new("x").with_child(Tree::new("parent").with_child(Tree::new("sibling")));
        assert_eq!(tree, expected);
    }

    #[test]
    #[ignore]
    fn can_reroot_a_tree_with_a_parent_and_many_siblings() {
        let mut tree = Tree::new("parent")
            .with_child(Tree::new("a"))
            .with_child(Tree::new("x"))
            .with_child(Tree::new("b"))
            .with_child(Tree::new("c"));
        let from = "x";
        assert!(tree.pov_from(&from));
        let expected = Tree::new("x").with_child(
            Tree::new("parent")
                .with_child(Tree::new("a"))
                .with_child(Tree::new("b"))
                .with_child(Tree::new("c")),
        );
        assert_eq!(tree, expected);
    }

    #[test]
    #[ignore]
    fn can_reroot_a_tree_with_new_root_deeply_nested_in_tree() {
        let mut tree = Tree::new("level-0").with_child(Tree::new("level-1").with_child(
            Tree::new("level-2").with_child(Tree::new("level-3").with_child(Tree::new("x"))),
        ));
        let from = "x";
        assert!(tree.pov_from(&from));
        let expected = Tree::new("x").with_child(Tree::new("level-3").with_child(
            Tree::new("level-2").with_child(Tree::new("level-1").with_child(Tree::new("level-0"))),
        ));
        assert_eq!(tree, expected);
    }

    #[test]
    #[ignore]
    fn moves_children_of_the_new_root_to_same_level_as_former_parent() {
        let mut tree = Tree::new("parent").with_child(
            Tree::new("x")
                .with_child(Tree::new("kid-0"))
                .with_child(Tree::new("kid-1")),
        );
        let from = "x";
        assert!(tree.pov_from(&from));
        let expected = Tree::new("x")
            .with_child(Tree::new("kid-0"))
            .with_child(Tree::new("kid-1"))
            .with_child(Tree::new("parent"));
        assert_eq!(tree, expected);
    }

    #[test]
    #[ignore]
    fn can_reroot_a_complex_tree_with_cousins() {
        let mut tree = Tree::new("grandparent")
            .with_child(
                Tree::new("parent")
                    .with_child(
                        Tree::new("x")
                            .with_child(Tree::new("kid-0"))
                            .with_child(Tree::new("kid-1")),
                    )
                    .with_child(Tree::new("sibling-0"))
                    .with_child(Tree::new("sibling-1")),
            )
            .with_child(
                Tree::new("uncle")
                    .with_child(Tree::new("cousin-0"))
                    .with_child(Tree::new("cousin-1")),
            );
        let from = "x";
        assert!(tree.pov_from(&from));
        let expected = Tree::new("x")
            .with_child(Tree::new("kid-1"))
            .with_child(Tree::new("kid-0"))
            .with_child(
                Tree::new("parent")
                    .with_child(Tree::new("sibling-0"))
                    .with_child(Tree::new("sibling-1"))
                    .with_child(
                        Tree::new("grandparent").with_child(
                            Tree::new("uncle")
                                .with_child(Tree::new("cousin-0"))
                                .with_child(Tree::new("cousin-1")),
                        ),
                    ),
            );
        assert_eq!(tree, expected);
    }

    #[test]
    #[ignore]
    fn errors_if_target_does_not_exist_in_a_singleton_tree() {
        let mut tree = Tree::new("x");
        let from = "nonexistent";
        assert!(!tree.pov_from(&from));
    }

    #[test]
    #[ignore]
    fn errors_if_target_does_not_exist_in_a_large_tree() {
        let mut tree = Tree::new("parent")
            .with_child(
                Tree::new("x")
                    .with_child(Tree::new("kid-0"))
                    .with_child(Tree::new("kid-1")),
            )
            .with_child(Tree::new("sibling-0"))
            .with_child(Tree::new("sibling-1"));
        let from = "nonexistent";
        assert!(!tree.pov_from(&from));
    }
}

/// Given two nodes, find the path between them
mod path_to {
    use pov::*;
    use pretty_assertions::assert_eq;

    #[test]
    #[ignore]
    fn can_find_path_to_parent() {
        let mut tree = Tree::new("parent")
            .with_child(Tree::new("x"))
            .with_child(Tree::new("sibling"));
        let from = "x";
        let to = "parent";
        let result = tree.path_to(&from, &to);
        let expected = Some(vec![&"x", &"parent"]);
        assert_eq!(result, expected);
    }

    #[test]
    #[ignore]
    fn can_find_path_to_sibling() {
        let mut tree = Tree::new("parent")
            .with_child(Tree::new("a"))
            .with_child(Tree::new("x"))
            .with_child(Tree::new("b"))
            .with_child(Tree::new("c"));
        let from = "x";
        let to = "b";
        let result = tree.path_to(&from, &to);
        let expected = Some(vec![&"x", &"parent", &"b"]);
        assert_eq!(result, expected);
    }

    #[test]
    #[ignore]
    fn can_find_path_to_cousin() {
        let mut tree = Tree::new("grandparent")
            .with_child(
                Tree::new("parent")
                    .with_child(
                        Tree::new("x")
                            .with_child(Tree::new("kid-0"))
                            .with_child(Tree::new("kid-1")),
                    )
                    .with_child(Tree::new("sibling-0"))
                    .with_child(Tree::new("sibling-1")),
            )
            .with_child(
                Tree::new("uncle")
                    .with_child(Tree::new("cousin-0"))
                    .with_child(Tree::new("cousin-1")),
            );
        let from = "x";
        let to = "cousin-1";
        let result = tree.path_to(&from, &to);
        let expected = Some(vec![&"x", &"parent", &"grandparent", &"uncle", &"cousin-1"]);
        assert_eq!(result, expected);
    }

    #[test]
    #[ignore]
    fn can_find_path_not_involving_root() {
        let mut tree = Tree::new("grandparent").with_child(
            Tree::new("parent")
                .with_child(Tree::new("x"))
                .with_child(Tree::new("sibling-0"))
                .with_child(Tree::new("sibling-1")),
        );
        let from = "x";
        let to = "sibling-1";
        let result = tree.path_to(&from, &to);
        let expected = Some(vec![&"x", &"parent", &"sibling-1"]);
        assert_eq!(result, expected);
    }

    #[test]
    #[ignore]
    fn can_find_path_from_nodes_other_than_x() {
        let mut tree = Tree::new("parent")
            .with_child(Tree::new("a"))
            .with_child(Tree::new("x"))
            .with_child(Tree::new("b"))
            .with_child(Tree::new("c"));
        let from = "a";
        let to = "c";
        let result = tree.path_to(&from, &to);
        let expected = Some(vec![&"a", &"parent", &"c"]);
        assert_eq!(result, expected);
    }

    #[test]
    #[ignore]
    fn errors_if_destination_does_not_exist() {
        let mut tree = Tree::new("parent")
            .with_child(
                Tree::new("x")
                    .with_child(Tree::new("kid-0"))
                    .with_child(Tree::new("kid-1")),
            )
            .with_child(Tree::new("sibling-0"))
            .with_child(Tree::new("sibling-1"));
        let from = "x";
        let to = "nonexistent";
        let result = tree.path_to(&from, &to);
        let expected: Option<Vec<_>> = None;
        assert_eq!(result, expected);
    }

    #[test]
    #[ignore]
    fn errors_if_source_does_not_exist() {
        let mut tree = Tree::new("parent")
            .with_child(
                Tree::new("x")
                    .with_child(Tree::new("kid-0"))
                    .with_child(Tree::new("kid-1")),
            )
            .with_child(Tree::new("sibling-0"))
            .with_child(Tree::new("sibling-1"));
        let from = "nonexistent";
        let to = "x";
        let result = tree.path_to(&from, &to);
        let expected: Option<Vec<_>> = None;
        assert_eq!(result, expected);
    }
}
