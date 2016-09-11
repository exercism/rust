extern crate pascals_triangle;

use pascals_triangle::*;

#[test]
fn no_rows() {
    let pt = PascalsTriangle::new(0);
    let expected: Vec<Vec<u32>> = Vec::new();
    assert_eq!(expected, pt.rows());
}

#[test]
fn one_row() {
    let pt = PascalsTriangle::new(1);
    let expected: Vec<Vec<u32>> = vec![vec![1]];
    assert_eq!(expected, pt.rows());
}

#[test]
fn two_rows() {
    let pt = PascalsTriangle::new(2);
    let expected: Vec<Vec<u32>> = vec![vec![1], vec![1, 1]];
    assert_eq!(expected, pt.rows());
}

#[test]
fn three_rows() {
    let pt = PascalsTriangle::new(3);
    let expected: Vec<Vec<u32>> = vec![vec![1], vec![1, 1], vec![1, 2, 1]];
    assert_eq!(expected, pt.rows());
}

#[test]
fn last_of_four_rows() {
    let pt = PascalsTriangle::new(4);
    let expected: Vec<u32> = vec![1, 3, 3, 1];
    assert_eq!(expected, pt.rows().pop().unwrap());
}
