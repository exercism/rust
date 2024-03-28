use matrix::*;

#[test]
fn extract_row_from_one_number_matrix() {
    let matrix = Matrix::new("1");
    assert_eq!(matrix.row(1), Some(vec![1]));
}

#[test]
#[ignore]
fn can_extract_row() {
    let matrix = Matrix::new("1 2\n3 4");
    assert_eq!(matrix.row(2), Some(vec![3, 4]));
}

#[test]
#[ignore]
fn extract_row_where_numbers_have_different_widths() {
    let matrix = Matrix::new("1 2\n10 20");
    assert_eq!(matrix.row(2), Some(vec![10, 20]));
}

#[test]
#[ignore]
fn can_extract_row_from_non_square_matrix_with_no_corresponding_column() {
    let matrix = Matrix::new("1 2 3\n4 5 6\n7 8 9\n8 7 6");
    assert_eq!(matrix.row(4), Some(vec![8, 7, 6]));
}

#[test]
#[ignore]
fn extract_column_from_one_number_matrix() {
    let matrix = Matrix::new("1");
    assert_eq!(matrix.column(1), Some(vec![1]));
}

#[test]
#[ignore]
fn can_extract_column() {
    let matrix = Matrix::new("1 2 3\n4 5 6\n7 8 9");
    assert_eq!(matrix.column(3), Some(vec![3, 6, 9]));
}

#[test]
#[ignore]
fn can_extract_column_from_non_square_matrix_with_no_corresponding_row() {
    let matrix = Matrix::new("1 2 3 4\n5 6 7 8\n9 8 7 6");
    assert_eq!(matrix.column(4), Some(vec![4, 8, 6]));
}

#[test]
#[ignore]
fn extract_column_where_numbers_have_different_widths() {
    let matrix = Matrix::new("89 1903 3\n18 3 1\n9 4 800");
    assert_eq!(matrix.column(2), Some(vec![1903, 3, 4]));
}

#[test]
#[ignore]
fn cannot_extract_row_with_no_corresponding_row_in_matrix() {
    let matrix = Matrix::new("1 2 3\n4 5 6\n7 8 9");
    assert_eq!(matrix.row(4), None);
}

#[test]
#[ignore]
fn cannot_extract_column_with_no_corresponding_column_in_matrix() {
    let matrix = Matrix::new("1 2 3\n4 5 6\n7 8 9");
    assert_eq!(matrix.column(4), None);
}
