extern crate pascals_triangle;

use pascals_triangle::*;

#[test]
fn no_rows() {
    let pt = PascalsTriangle::new(0);
    let expected: Vec<Vec<u32>> = Vec::new();
    assert_eq!(expected, pt.rows());
}

#[test]
#[ignore]
fn one_row() {
    let pt = PascalsTriangle::new(1);
    let expected: Vec<Vec<u32>> = vec![vec![1]];
    assert_eq!(expected, pt.rows());
}

#[test]
#[ignore]
fn two_rows() {
    let pt = PascalsTriangle::new(2);
    let expected: Vec<Vec<u32>> = vec![vec![1], vec![1, 1]];
    assert_eq!(expected, pt.rows());
}

#[test]
#[ignore]
fn three_rows() {
    let pt = PascalsTriangle::new(3);
    let expected: Vec<Vec<u32>> = vec![vec![1], vec![1, 1], vec![1, 2, 1]];
    assert_eq!(expected, pt.rows());
}

#[test]
#[ignore]
fn last_of_four_rows() {
    let pt = PascalsTriangle::new(4);
    let expected: Vec<u32> = vec![1, 3, 3, 1];
    assert_eq!(expected, pt.rows().pop().unwrap());
}

#[test]
#[ignore]
fn last_of_seven_rows() {
    let pt = PascalsTriangle::new(7);
    let expected: Vec<u32> = vec![1, 6, 15, 20, 15, 6, 1];
    assert_eq!(expected, pt.rows().pop().unwrap());
}

#[test]
#[ignore]
fn last_of_twenty_rows() {
    let pt = PascalsTriangle::new(21);
    let expected: Vec<u32> = vec![1,20,190,1140,4845,15504,38760,77520,125970,167960,184756,167960,125970,77520,38760,15504,4845,1140,190,20,1];
    assert_eq!(expected, pt.rows().pop().unwrap());
}
