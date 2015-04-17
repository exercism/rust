extern crate dna;

#[test]
#[ignore]
fn test_no_difference_between_empty_strands() {
    assert_eq!(dna::hamming_distance("", ""), 0);
}

#[test]
#[ignore]
fn test_no_difference_between_identical_strands() {
    assert_eq!(dna::hamming_distance("GGACTGA", "GGACTGA"), 0);
}

#[test]
#[ignore]
fn test_complete_hamming_distance_in_small_strand() {
    assert_eq!(dna::hamming_distance("ACT", "GGA"), 3);
}

#[test]
#[ignore]
fn test_small_hamming_distance_in_the_middle_somewhere() {
    assert_eq!(dna::hamming_distance("GGACG", "GGTCG"), 1);
}

#[test]
#[ignore]
fn test_larger_distance() {
    assert_eq!(dna::hamming_distance("ACCAGGG", "ACTATGG"), 2);
}
