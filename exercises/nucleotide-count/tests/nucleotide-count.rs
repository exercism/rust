extern crate nucleotide_count as dna;

use std::collections::HashMap;

fn check_dna(s: &str, pairs: &[(char, usize)]) {
    // The reason for the awkward code in here is to ensure that the failure
    // message for assert_eq! is as informative as possible. A simpler
    // solution would simply check the length of the map, and then
    // check for the presence and value of each key in the given pairs vector.
    let mut m: HashMap<char, usize> = dna::nucleotide_counts(s).unwrap();
    for &(k, v) in pairs.iter() {
        assert_eq!((k, m.remove(&k).unwrap()), (k, v));
    }
    // may fail with a message that clearly shows all extra pairs in the map
    assert_eq!(m.iter().collect::<Vec<(&char,&usize)>>(), vec!());
}

#[test]
fn count_returns_result() {
    assert!(dna::count('A', "").is_ok());
}

#[test]
#[ignore]
fn test_count_empty() {
    assert_eq!(dna::count('A', "").unwrap(), 0);
}

#[test]
#[ignore]
fn count_invalid_nucleotide() {
    assert!(dna::count('X', "A").is_err());
}

#[test]
#[ignore]
fn count_invalid_dna() {
    assert!(dna::count('A', "AX").is_err());
}

#[test]
#[ignore]
fn test_count_repetitive_cytosine() {
    assert_eq!(dna::count('C', "CCCCC").unwrap(), 5);
}

#[test]
#[ignore]
fn test_count_only_thymine() {
    assert_eq!(dna::count('T', "GGGGGTAACCCGG").unwrap(), 1);
}

#[test]
#[ignore]
fn counts_returns_result() {
    assert!(dna::nucleotide_counts("ACGT").is_ok());
}

#[test]
#[ignore]
fn test_nucleotide_count_empty() {
    check_dna(
        "",
        &[('A', 0), ('T', 0), ('C', 0), ('G', 0)]);
}

#[test]
#[ignore]
fn test_nucleotide_count_only_guanine() {
    check_dna(
        "GGGGGGGG",
        &[('A', 0), ('T', 0), ('C', 0), ('G', 8)]);
}

#[test]
#[ignore]
fn test_nucleotide_count_counts_all() {
    check_dna(
        "AGCTTTTCATTCTGACTGCAACGGGCAATATGTCTCTGTGTGGATTAAAAAAA\
         GAGTGTCTGATAGCAGC",
        &[('A', 20), ('T', 21), ('C', 12), ('G', 17)]);
}

#[test]
#[ignore]
fn counts_invalid_nucleotide_results_in_err() {
    assert!(dna::nucleotide_counts("GGXXX").is_err());
}
