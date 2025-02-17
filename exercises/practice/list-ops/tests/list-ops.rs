use list_ops::*;

mod append {
    use super::*;

    #[test]
    fn empty_lists() {
        let list1 = vec![0i32; 0].into_iter();
        let list2 = vec![0i32; 0].into_iter();
        let output = append(list1, list2);

        let expected = vec![];

        assert_eq!(output.collect::<Vec<_>>(), expected);
    }

    #[test]
    #[ignore]
    fn list_to_empty_list() {
        let list1 = vec![0i32; 0].into_iter();
        let list2 = vec![1, 2, 3, 4].into_iter();
        let output = append(list1, list2);

        let expected = vec![1, 2, 3, 4];

        assert_eq!(output.collect::<Vec<_>>(), expected);
    }

    #[test]
    #[ignore]
    fn empty_list_to_list() {
        let list1 = vec![1, 2, 3, 4].into_iter();
        let list2 = vec![0i32; 0].into_iter();
        let output = append(list1, list2);

        let expected = vec![1, 2, 3, 4];

        assert_eq!(output.collect::<Vec<_>>(), expected);
    }

    #[test]
    #[ignore]
    fn non_empty_lists() {
        let list1 = vec![1, 2].into_iter();
        let list2 = vec![2, 3, 4, 5].into_iter();
        let output = append(list1, list2);

        let expected = vec![1, 2, 2, 3, 4, 5];

        assert_eq!(output.collect::<Vec<_>>(), expected);
    }
}

#[allow(clippy::zero_repeat_side_effects)]
mod concat {
    use super::*;

    #[test]
    #[ignore]
    fn empty_list() {
        let lists = vec![vec![0i32; 0]; 0].into_iter().map(Vec::into_iter);
        let output = concat(lists);

        let expected = vec![];

        assert_eq!(output.collect::<Vec<_>>(), expected);
    }

    #[test]
    #[ignore]
    fn list_of_lists() {
        let lists = vec![vec![1, 2], vec![3], vec![], vec![4, 5, 6]]
            .into_iter()
            .map(Vec::into_iter);
        let output = concat(lists);

        let expected = vec![1, 2, 3, 4, 5, 6];

        assert_eq!(output.collect::<Vec<_>>(), expected);
    }

    #[test]
    #[ignore]
    fn list_of_nested_lists() {
        let lists = vec![
            vec![vec![1], vec![2]],
            vec![vec![3]],
            vec![vec![]],
            vec![vec![4, 5, 6]],
        ]
        .into_iter()
        .map(Vec::into_iter);
        let output = concat(lists);

        let expected = vec![vec![1], vec![2], vec![3], vec![], vec![4, 5, 6]];

        assert_eq!(output.collect::<Vec<_>>(), expected);
    }
}

mod filter {
    use super::*;

    #[test]
    #[ignore]
    fn empty_list() {
        let list = vec![0i32; 0].into_iter();
        let output = filter(list, |x| x % 2 == 1);

        let expected = vec![];

        assert_eq!(output.collect::<Vec<_>>(), expected);
    }

    #[test]
    #[ignore]
    fn non_empty_list() {
        let list = vec![1, 2, 3, 5].into_iter();
        let output = filter(list, |x| x % 2 == 1);

        let expected = vec![1, 3, 5];

        assert_eq!(output.collect::<Vec<_>>(), expected);
    }
}

mod length {
    use super::*;

    #[test]
    #[ignore]
    fn empty_list() {
        let list = vec![0i32; 0].into_iter();
        let output = length(list);

        let expected = 0;

        assert_eq!(output, expected);
    }

    #[test]
    #[ignore]
    fn non_empty_list() {
        let list = vec![1, 2, 3, 4].into_iter();
        let output = length(list);

        let expected = 4;

        assert_eq!(output, expected);
    }
}

mod map {
    use super::*;

    #[test]
    #[ignore]
    fn empty_list() {
        let list = vec![0i32; 0].into_iter();
        let output = map(list, |x| x + 1);

        let expected = vec![];

        assert_eq!(output.collect::<Vec<_>>(), expected);
    }

    #[test]
    #[ignore]
    fn non_empty_list() {
        let list = vec![1, 3, 5, 7].into_iter();
        let output = map(list, |x| x + 1);

        let expected = vec![2, 4, 6, 8];

        assert_eq!(output.collect::<Vec<_>>(), expected);
    }
}

mod foldl {
    use super::*;

    #[test]
    #[ignore]
    fn empty_list() {
        let list = vec![0.0f64; 0].into_iter();
        let initial = 2.0;
        let output = foldl(list, initial, |acc, el| el * acc);

        let expected = 2.0;

        assert_eq!(output, expected);
    }

    #[test]
    #[ignore]
    fn direction_independent_function_applied_to_non_empty_list() {
        let list = vec![1.0, 2.0, 3.0, 4.0].into_iter();
        let initial = 5.0;
        let output = foldl(list, initial, |acc, el| el + acc);

        let expected = 15.0;

        assert_eq!(output, expected);
    }

    #[test]
    #[ignore]
    fn direction_dependent_function_applied_to_non_empty_list() {
        let list = vec![1.0, 2.0, 3.0, 4.0].into_iter();
        let initial = 24.0;
        let output = foldl(list, initial, |acc, el| el / acc);

        let expected = 64.0;

        assert_eq!(output, expected);
    }
}

mod foldr {
    use super::*;

    #[test]
    #[ignore]
    fn empty_list() {
        let list = vec![0.0f64; 0].into_iter();
        let initial = 2.0;
        let output = foldr(list, initial, |acc, el| el * acc);

        let expected = 2.0;

        assert_eq!(output, expected);
    }

    #[test]
    #[ignore]
    fn direction_independent_function_applied_to_non_empty_list() {
        let list = vec![1.0, 2.0, 3.0, 4.0].into_iter();
        let initial = 5.0;
        let output = foldr(list, initial, |acc, el| el + acc);

        let expected = 15.0;

        assert_eq!(output, expected);
    }

    #[test]
    #[ignore]
    fn direction_dependent_function_applied_to_non_empty_list() {
        let list = vec![1.0, 2.0, 3.0, 4.0].into_iter();
        let initial = 24.0;
        let output = foldr(list, initial, |acc, el| el / acc);

        let expected = 9.0;

        assert_eq!(output, expected);
    }
}

mod reverse {
    use super::*;

    #[test]
    #[ignore]
    fn empty_list() {
        let list = vec![0i32; 0].into_iter();
        let output = reverse(list);

        let expected = vec![];

        assert_eq!(output.collect::<Vec<_>>(), expected);
    }

    #[test]
    #[ignore]
    fn non_empty_list() {
        let list = vec![1, 3, 5, 7].into_iter();
        let output = reverse(list);

        let expected = vec![7, 5, 3, 1];

        assert_eq!(output.collect::<Vec<_>>(), expected);
    }

    #[test]
    #[ignore]
    fn list_of_lists_is_not_flattened() {
        let list = vec![vec![1, 2], vec![3], vec![], vec![4, 5, 6]]
            .into_iter()
            .map(Vec::into_iter);
        let output = reverse(list);

        let expected = vec![vec![4, 5, 6], vec![], vec![3], vec![1, 2]];

        assert_eq!(
            output
                .map(|subiter| subiter.collect::<Vec<_>>())
                .collect::<Vec<_>>(),
            expected
        );
    }
}
