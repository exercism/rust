use spiral_matrix::*;

#[test]
fn empty_spiral() {
    let expected: Vec<Vec<u32>> = Vec::new();
    assert_eq!(spiral_matrix(0), expected);
}

#[test]
#[ignore]
fn size_one_spiral() {
    let mut expected: Vec<Vec<u32>> = Vec::new();
    expected.push(vec![1]);
    assert_eq!(spiral_matrix(1), expected);
}
#[test]
#[ignore]
fn size_two_spiral() {
    let mut expected: Vec<Vec<u32>> = Vec::new();
    expected.push(vec![1, 2]);
    expected.push(vec![4, 3]);
    assert_eq!(spiral_matrix(2), expected);
}

#[test]
#[ignore]
fn size_three_spiral() {
    let mut expected: Vec<Vec<u32>> = Vec::new();
    expected.push(vec![1, 2, 3]);
    expected.push(vec![8, 9, 4]);
    expected.push(vec![7, 6, 5]);
    assert_eq!(spiral_matrix(3), expected);
}
#[test]
#[ignore]
fn size_four_spiral() {
    let mut expected: Vec<Vec<u32>> = Vec::new();
    expected.push(vec![1, 2, 3, 4]);
    expected.push(vec![12, 13, 14, 5]);
    expected.push(vec![11, 16, 15, 6]);
    expected.push(vec![10, 9, 8, 7]);
    assert_eq!(spiral_matrix(4), expected);
}
#[test]
#[ignore]
fn size_five_spiral() {
    let mut expected: Vec<Vec<u32>> = Vec::new();
    expected.push(vec![1, 2, 3, 4, 5]);
    expected.push(vec![16, 17, 18, 19, 6]);
    expected.push(vec![15, 24, 25, 20, 7]);
    expected.push(vec![14, 23, 22, 21, 8]);
    expected.push(vec![13, 12, 11, 10, 9]);
    assert_eq!(spiral_matrix(5), expected);
}
