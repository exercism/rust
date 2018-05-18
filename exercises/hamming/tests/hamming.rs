extern crate hamming;

#[test]
fn test_no_difference_between_empty_strands() {
    assert_eq!(hamming::hamming_distance("", ""), Some(0));
}

#[test]
#[ignore]
fn test_no_difference_between_identical_strands() {
    assert_eq!(hamming::hamming_distance("GGACTGA", "GGACTGA"), Some(0));
}

#[test]
#[ignore]
fn test_complete_hamming_distance_in_small_strand() {
    assert_eq!(hamming::hamming_distance("ACT", "GGA"), Some(3));
}

#[test]
#[ignore]
fn test_small_hamming_distance_in_the_middle_somewhere() {
    assert_eq!(hamming::hamming_distance("GGACG", "GGTCG"), Some(1));
}

#[test]
#[ignore]
fn test_larger_distance() {
    assert_eq!(hamming::hamming_distance("ACCAGGG", "ACTATGG"), Some(2));
}

#[test]
#[ignore]
fn test_first_string_is_longer() {
    assert_eq!(hamming::hamming_distance("AAA", "AA"), None);
}

#[test]
#[ignore]
fn test_second_string_is_longer() {
    assert_eq!(hamming::hamming_distance("A", "AA"), None);
}
