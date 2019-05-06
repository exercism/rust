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
    assert_eq!(Some(expected), pt.rows().pop());
}

#[test]
#[ignore]
fn five_rows() {
    let pt = PascalsTriangle::new(5);
    let expected: Vec<Vec<u32>> = vec![
        vec![1],
        vec![1, 1],
        vec![1, 2, 1],
        vec![1, 3, 3, 1],
        vec![1, 4, 6, 4, 1],
    ];
    assert_eq!(expected, pt.rows());
}

#[test]
#[ignore]
fn six_rows() {
    let pt = PascalsTriangle::new(6);
    let expected: Vec<Vec<u32>> = vec![
        vec![1],
        vec![1, 1],
        vec![1, 2, 1],
        vec![1, 3, 3, 1],
        vec![1, 4, 6, 4, 1],
        vec![1, 5, 10, 10, 5, 1],
    ];
    assert_eq!(expected, pt.rows());
}

#[test]
#[ignore]
fn seven_rows() {
    let pt = PascalsTriangle::new(7);
    let expected: Vec<Vec<u32>> = vec![
        vec![1],
        vec![1, 1],
        vec![1, 2, 1],
        vec![1, 3, 3, 1],
        vec![1, 4, 6, 4, 1],
        vec![1, 5, 10, 10, 5, 1],
        vec![1, 6, 15, 20, 15, 6, 1],
    ];
    assert_eq!(expected, pt.rows());
}

#[test]
#[ignore]
fn ten_rows() {
    let pt = PascalsTriangle::new(10);
    let expected: Vec<Vec<u32>> = vec![
        vec![1],
        vec![1, 1],
        vec![1, 2, 1],
        vec![1, 3, 3, 1],
        vec![1, 4, 6, 4, 1],
        vec![1, 5, 10, 10, 5, 1],
        vec![1, 6, 15, 20, 15, 6, 1],
        vec![1, 7, 21, 35, 35, 21, 7, 1],
        vec![1, 8, 28, 56, 70, 56, 28, 8, 1],
        vec![1, 9, 36, 84, 126, 126, 84, 36, 9, 1],
    ];
    assert_eq!(expected, pt.rows());
}
