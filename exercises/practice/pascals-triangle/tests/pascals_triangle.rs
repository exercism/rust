use pascals_triangle::PascalsTriangle;

#[test]
fn zero_rows() {
    let pt = PascalsTriangle::new(0);
    let expected: Vec<Vec<u32>> = vec![];
    assert_eq!(pt.rows(), expected);
}

#[test]
#[ignore]
fn single_row() {
    let pt = PascalsTriangle::new(1);
    let expected: Vec<Vec<u32>> = vec![vec![1]];
    assert_eq!(pt.rows(), expected);
}

#[test]
#[ignore]
fn two_rows() {
    let pt = PascalsTriangle::new(2);
    let expected: Vec<Vec<u32>> = vec![vec![1], vec![1, 1]];
    assert_eq!(pt.rows(), expected);
}

#[test]
#[ignore]
fn three_rows() {
    let pt = PascalsTriangle::new(3);
    let expected: Vec<Vec<u32>> = vec![vec![1], vec![1, 1], vec![1, 2, 1]];
    assert_eq!(pt.rows(), expected);
}

#[test]
#[ignore]
fn four_rows() {
    let pt = PascalsTriangle::new(4);
    let expected: Vec<Vec<u32>> = vec![vec![1], vec![1, 1], vec![1, 2, 1], vec![1, 3, 3, 1]];
    assert_eq!(pt.rows(), expected);
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
    assert_eq!(pt.rows(), expected);
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
    assert_eq!(pt.rows(), expected);
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
    assert_eq!(pt.rows(), expected);
}
