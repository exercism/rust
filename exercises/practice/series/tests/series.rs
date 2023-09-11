use series::*;

#[test]
fn with_zero_length() {
    let expected = vec!["".to_string(); 6];
    assert_eq!(series("92017", 0), expected);
}

#[test]
#[ignore]
fn with_length_2() {
    let expected = vec![
        "92".to_string(),
        "20".to_string(),
        "01".to_string(),
        "17".to_string(),
    ];
    assert_eq!(series("92017", 2), expected);
}

#[test]
#[ignore]
fn with_numbers_length() {
    let expected = vec!["92017".to_string()];
    assert_eq!(series("92017", 5), expected);
}

#[test]
#[ignore]
fn too_long() {
    let expected: Vec<String> = vec![];
    assert_eq!(series("92017", 6), expected);
}

#[test]
#[ignore]
fn way_too_long() {
    let expected: Vec<String> = vec![];
    assert_eq!(series("92017", 42), expected);
}
