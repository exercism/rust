use nucleotide_count as dna;

use std::collections::HashMap;

fn process_nucleotidecounts_case(s: &str, pairs: &[(char, usize)]) {
    // The reason for the awkward code in here is to ensure that the failure
    // message for assert_eq! is as informative as possible. A simpler
    // solution would simply check the length of the map, and then
    // check for the presence and value of each key in the given pairs vector.
    let mut m: HashMap<char, usize> = dna::nucleotide_counts(s).unwrap();
    for &(k, v) in pairs.iter() {
        assert_eq!((k, m.remove(&k)), (k, Some(v)));
    }

    // may fail with a message that clearly shows all extra pairs in the map
    assert_eq!(m.iter().collect::<Vec<(&char, &usize)>>(), vec![]);
}

#[test]
fn count_returns_result() {
    assert!(dna::count('A', "").is_ok());
}

#[test]
#[ignore]
fn test_count_empty() {
    assert_eq!(dna::count('A', ""), Ok(0));
}

#[test]
#[ignore]
fn count_invalid_nucleotide() {
    assert_eq!(dna::count('X', "A"), Err('X'));
}

#[test]
#[ignore]
fn count_invalid_dna() {
    assert_eq!(dna::count('A', "AX"), Err('X'));
}

#[test]
#[ignore]
fn test_count_repetitive_cytosine() {
    assert_eq!(dna::count('C', "CCCCC"), Ok(5));
}

#[test]
#[ignore]
fn test_count_only_thymine() {
    assert_eq!(dna::count('T', "GGGGGTAACCCGG"), Ok(1));
}

#[test]
#[ignore]
fn counts_returns_result() {
    assert!(dna::nucleotide_counts("ACGT").is_ok());
}

#[test]
#[ignore]
fn test_empty_strand() {
    process_nucleotidecounts_case("", &[('A', 0), ('T', 0), ('C', 0), ('G', 0)]);
}

#[test]
#[ignore]
/// can count one nucleotide in single-character input
fn test_can_count_one_nucleotide_in_singlecharacter_input() {
    process_nucleotidecounts_case("G", &[('A', 0), ('C', 0), ('G', 1), ('T', 0)]);
}

#[test]
#[ignore]
fn test_strand_with_repeated_nucleotide() {
    process_nucleotidecounts_case("GGGGGGG", &[('A', 0), ('T', 0), ('C', 0), ('G', 7)]);
}

#[test]
#[ignore]
/// strand with multiple nucleotides
fn test_strand_with_multiple_nucleotides() {
    process_nucleotidecounts_case(
        "AGCTTTTCATTCTGACTGCAACGGGCAATATGTCTCTGTGTGGATTAAAAAAAGAGTGTCTGATAGCAGC",
        &[('A', 20), ('T', 21), ('C', 12), ('G', 17)],
    );
}

#[test]
#[ignore]
fn counts_invalid_nucleotide_results_in_err() {
    assert_eq!(dna::nucleotide_counts("GGXXX"), Err('X'));
}

#[test]
#[ignore]
/// strand with invalid nucleotides
fn test_strand_with_invalid_nucleotides() {
    assert_eq!(dna::nucleotide_counts("AGXXACT"), Err('X'),);
}
