extern crate saddle_points;

use saddle_points::*;

#[test]
fn test_identify_single_saddle_point() {
    let input = vec![vec![9, 8, 7], vec![5, 3, 2], vec![6, 6, 7]];
    assert_eq!(vec![(1, 0)], find_saddle_points(&input));
}

#[test]
#[ignore]
fn test_identify_empty_matrix() {
    let input = vec![vec![], vec![], vec![]];
    let expected: Vec<(usize, usize)> = Vec::new();
    assert_eq!(expected, find_saddle_points(&input));
}

#[test]
#[ignore]
fn test_identify_lack_of_saddle_point() {
    let input = vec![vec![1, 2, 3], vec![3, 1, 2], vec![2, 3, 1]];
    let expected: Vec<(usize, usize)> = Vec::new();
    assert_eq!(expected, find_saddle_points(&input));
}

#[test]
#[ignore]
fn test_multiple_saddle_point() {
    let input = vec![vec![4, 5, 4], vec![3, 5, 5], vec![1, 5, 4]];
    assert_eq!(vec![(0, 1), (1, 1), (2, 1)], find_saddle_points(&input));
}

#[test]
#[ignore]
fn test_identify_bottom_right_saddle_point() {
    let input = vec![vec![8, 7, 9], vec![6, 7, 6], vec![3, 2, 5]];
    assert_eq!(vec![(2, 2)], find_saddle_points(&input));
}

#[test]
#[ignore]
fn test_non_square_matrix_high() {
    let input = vec![vec![1, 5], vec![3, 6], vec![2, 7], vec![3, 8]];
    assert_eq!(vec![(0, 1)], find_saddle_points(&input));
}

#[test]
#[ignore]
fn test_non_square_matrix_wide() {
    let input = vec![vec![8, 7, 10, 7, 9], vec![8, 7, 13, 7, 9]];
    assert_eq!(vec![(0, 2)], find_saddle_points(&input));
}

#[test]
#[ignore]
fn test_vector_matrix() {
    let input = vec![vec![1], vec![3], vec![2], vec![3]];
    assert_eq!(vec![(0, 0)], find_saddle_points(&input));
}
