use hamming::*;

#[test]
fn empty_strands() {
    assert_eq!(hamming_distance("", ""), Some(0));
}

#[test]
#[ignore]
fn single_letter_identical_strands() {
    assert_eq!(hamming_distance("A", "A"), Some(0));
}

#[test]
#[ignore]
fn single_letter_different_strands() {
    assert_eq!(hamming_distance("G", "T"), Some(1));
}

#[test]
#[ignore]
fn long_identical_strands() {
    assert_eq!(hamming_distance("GGACTGAAATCTG", "GGACTGAAATCTG"), Some(0));
}

#[test]
#[ignore]
fn long_different_strands() {
    assert_eq!(hamming_distance("GGACGGATTCTG", "AGGACGGATTCT"), Some(9));
}

#[test]
#[ignore]
fn disallow_first_strand_longer() {
    assert_eq!(hamming_distance("AATG", "AAA"), None);
}

#[test]
#[ignore]
fn disallow_second_strand_longer() {
    assert_eq!(hamming_distance("ATA", "AGTG"), None);
}

#[test]
#[ignore]
fn disallow_empty_first_strand() {
    assert_eq!(hamming_distance("", "G"), None);
}

#[test]
#[ignore]
fn disallow_empty_second_strand() {
    assert_eq!(hamming_distance("G", ""), None);
}
