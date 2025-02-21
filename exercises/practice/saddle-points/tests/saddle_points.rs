use saddle_points::*;

#[test]
fn can_identify_single_saddle_point() {
    let input = &[vec![9, 8, 7], vec![5, 3, 2], vec![6, 6, 7]];
    let mut output = find_saddle_points(input);
    output.sort_unstable();
    let expected = &[(1, 0)];
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn can_identify_that_empty_matrix_has_no_saddle_points() {
    let input = &[vec![]];
    let mut output = find_saddle_points(input);
    output.sort_unstable();
    let expected = &[];
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn can_identify_lack_of_saddle_points_when_there_are_none() {
    let input = &[vec![1, 2, 3], vec![3, 1, 2], vec![2, 3, 1]];
    let mut output = find_saddle_points(input);
    output.sort_unstable();
    let expected = &[];
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn can_identify_multiple_saddle_points_in_a_column() {
    let input = &[vec![4, 5, 4], vec![3, 5, 5], vec![1, 5, 4]];
    let mut output = find_saddle_points(input);
    output.sort_unstable();
    let expected = &[(0, 1), (1, 1), (2, 1)];
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn can_identify_multiple_saddle_points_in_a_row() {
    let input = &[vec![6, 7, 8], vec![5, 5, 5], vec![7, 5, 6]];
    let mut output = find_saddle_points(input);
    output.sort_unstable();
    let expected = &[(1, 0), (1, 1), (1, 2)];
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn can_identify_saddle_point_in_bottom_right_corner() {
    let input = &[vec![8, 7, 9], vec![6, 7, 6], vec![3, 2, 5]];
    let mut output = find_saddle_points(input);
    output.sort_unstable();
    let expected = &[(2, 2)];
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn can_identify_saddle_points_in_a_non_square_matrix() {
    let input = &[vec![3, 1, 3], vec![3, 2, 4]];
    let mut output = find_saddle_points(input);
    output.sort_unstable();
    let expected = &[(0, 0), (0, 2)];
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn can_identify_that_saddle_points_in_a_single_column_matrix_are_those_with_the_minimum_value() {
    let input = &[vec![2], vec![1], vec![4], vec![1]];
    let mut output = find_saddle_points(input);
    output.sort_unstable();
    let expected = &[(1, 0), (3, 0)];
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn can_identify_that_saddle_points_in_a_single_row_matrix_are_those_with_the_maximum_value() {
    let input = &[vec![2, 5, 3, 5]];
    let mut output = find_saddle_points(input);
    output.sort_unstable();
    let expected = &[(0, 1), (0, 3)];
    assert_eq!(output, expected);
}
