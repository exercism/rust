extern crate saddle_points;

use saddle_points::*;

#[test]
fn test_identify_single_saddle_point() {
    let input: &[&[u64]] = &[&[9, 8, 7],
                             &[5, 3, 2],
                             &[6, 6, 7]];
    assert_eq!(vec![(1,0)], find_saddle_points(input));
}

#[test]
#[ignore]
fn test_identify_empty_matrix() {
    let input: &[&[u64]] = &[&[]];
    let expected: Vec<(u64, u64)> = Vec::new();

    assert_eq!(expected, find_saddle_points(input));
}

#[test]
#[ignore]
fn test_identify_lack_of_saddle_point() {
    let input: &[&[u64]] = &[&[1, 2, 3],
                             &[3, 1, 2],
                             &[2, 3, 1]];
    let expected: Vec<(u64, u64)> = Vec::new();
    assert_eq!(expected, find_saddle_points(input));
}

#[test]
#[ignore]
fn test_identify_bottom_right_saddle_point() {
    let input: &[&[u64]] = &[&[8, 7, 9],
                             &[6, 7, 6],
                             &[3, 2, 5]];
    assert_eq!(vec![(2,2)], find_saddle_points(input));
}

#[test]
#[ignore]
fn test_non_square_matrix_high() {
    let input: &[&[u64]] = &[&[1, 5],
                             &[3, 6],
                             &[2, 7],
                             &[3, 8]];
    assert_eq!(vec![(0, 1)], find_saddle_points(input));
}

#[test]
#[ignore]
fn test_non_quadratic_matrix_wide() {
    let input: &[&[u64]] = &[&[8, 7, 10, 7, 9],
                             &[8, 7, 13, 7, 9]];
    assert_eq!(vec![(0, 2)], find_saddle_points(input));
}

#[test]
#[ignore]
fn test_vector_matrix() {
    let input: &[&[u64]] = &[&[1],
                             &[3],
                             &[2],
                             &[3]];
    assert_eq!(vec![(0, 0)], find_saddle_points(input));
}
