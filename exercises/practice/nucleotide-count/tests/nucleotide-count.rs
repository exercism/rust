use std::collections::HashMap;

use nucleotide_count::*;

#[test]
fn count_empty() {
    assert_eq!(count('A', ""), Ok(0));
}

#[test]
#[ignore]
fn count_invalid_nucleotide() {
    assert_eq!(count('X', "A"), Err('X'));
}

#[test]
#[ignore]
fn count_invalid_dna() {
    assert_eq!(count('A', "AX"), Err('X'));
}

#[test]
#[ignore]
fn count_repetitive_cytosine() {
    assert_eq!(count('C', "CCCCC"), Ok(5));
}

#[test]
#[ignore]
fn count_only_thymine() {
    assert_eq!(count('T', "GGGGGTAACCCGG"), Ok(1));
}

#[test]
#[ignore]
fn empty_strand() {
    let output = nucleotide_counts("");
    let mut expected = HashMap::new();
    expected.insert('A', 0);
    expected.insert('C', 0);
    expected.insert('G', 0);
    expected.insert('T', 0);
    assert_eq!(output, Ok(expected));
}

#[test]
#[ignore]
fn can_count_one_nucleotide_in_single_character_input() {
    let output = nucleotide_counts("G");
    let mut expected = HashMap::new();
    expected.insert('A', 0);
    expected.insert('C', 0);
    expected.insert('G', 1);
    expected.insert('T', 0);
    assert_eq!(output, Ok(expected));
}

#[test]
#[ignore]
fn strand_with_repeated_nucleotide() {
    let output = nucleotide_counts("GGGGGGG");
    let mut expected = HashMap::new();
    expected.insert('A', 0);
    expected.insert('C', 0);
    expected.insert('G', 7);
    expected.insert('T', 0);
    assert_eq!(output, Ok(expected));
}

#[test]
#[ignore]
fn strand_with_multiple_nucleotides() {
    let output =
        nucleotide_counts("AGCTTTTCATTCTGACTGCAACGGGCAATATGTCTCTGTGTGGATTAAAAAAAGAGTGTCTGATAGCAGC");
    let mut expected = HashMap::new();
    expected.insert('A', 20);
    expected.insert('C', 12);
    expected.insert('G', 17);
    expected.insert('T', 21);
    assert_eq!(output, Ok(expected));
}

#[test]
#[ignore]
fn strand_with_invalid_nucleotides() {
    let output = nucleotide_counts("AGXXACT");
    assert!(output.is_err());
}
