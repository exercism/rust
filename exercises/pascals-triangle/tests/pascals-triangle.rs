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
fn five_rows() {
    let pt = PascalsTriangle::new(5);
    let expected: Vec<Vec<u32>> = vec![vec![1],
                                       vec![1, 1],
                                       vec![1, 2, 1],
                                       vec![1, 3, 3, 1],
                                       vec![1, 4, 6, 4, 1]];
    assert_eq!(expected, pt.rows());
}

#[test]
#[ignore]
fn six_rows() {
    let pt = PascalsTriangle::new(6);
    let expected: Vec<Vec<u32>> = vec![vec![1],
                                       vec![1, 1],
                                       vec![1, 2, 1],
                                       vec![1, 3, 3, 1],
                                       vec![1, 4, 6, 4, 1],
                                       vec![1, 5, 10, 10, 5, 1]];
    assert_eq!(expected, pt.rows());
}

#[test]
#[ignore]
fn seven_rows() {
    let pt = PascalsTriangle::new(7);
    let expected: Vec<Vec<u32>> = vec![vec![1],
                                       vec![1, 1],
                                       vec![1, 2, 1],
                                       vec![1, 3, 3, 1],
                                       vec![1, 4, 6, 4, 1],
                                       vec![1, 5, 10, 10, 5, 1],
                                       vec![1, 6, 15, 20, 15, 6, 1]];
    assert_eq!(expected, pt.rows());
}

#[test]
#[ignore]
fn ten_rows() {
    let pt = PascalsTriangle::new(10);
    let expected: Vec<Vec<u32>> = vec![vec![1],
                                       vec![1, 1],
                                       vec![1, 2, 1],
                                       vec![1, 3, 3, 1],
                                       vec![1, 4, 6, 4, 1],
                                       vec![1, 5, 10, 10, 5, 1],
                                       vec![1, 6, 15, 20, 15, 6, 1],
                                       vec![1, 7, 21, 35, 35, 21, 7, 1],
                                       vec![1, 8, 28, 56, 70, 56, 28, 8, 1],
                                       vec![1, 9, 36, 84, 126, 126, 84, 36, 9, 1]];
    assert_eq!(expected, pt.rows());
}

#[test]
#[ignore]
#[should_panic]
#![deny(warnings)]
fn middle_numbers_getting_to_big_for_u32() {
    PascalsTriangle::new(36);
}

#[test]
#[ignore]
fn last_of_35_rows() {
    let pt = PascalsTriangle::new(35);
    let expected: Vec<u32> =
        vec![1, 34, 561, 5984, 46376, 278256, 1344904, 5379616, 18156204, 52451256, 131128140,
             286097760, 548354040, 927983760, 1391975640, 1855967520, 2203961430, 2333606220,
             2203961430, 1855967520, 1391975640, 927983760, 548354040, 286097760, 131128140,
             52451256, 18156204, 5379616, 1344904, 278256, 46376, 5984, 561, 34, 1];
    assert_eq!(expected, pt.rows().pop().unwrap());
}

#[test]
#[ignore]
fn last_of_four_rows() {
    let pt = PascalsTriangle::new(4);
    let expected: Vec<u32> = vec![1, 3, 3, 1];
    assert_eq!(expected, pt.rows().pop().unwrap());
}
