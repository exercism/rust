use rna_transcription::{Dna, Rna};

#[test]
fn empty_rna_sequence() {
    let input = "";
    let output = Dna::new(input).unwrap().into_rna();
    let expected = Rna::new("").unwrap();
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn rna_complement_of_cytosine_is_guanine() {
    let input = "C";
    let output = Dna::new(input).unwrap().into_rna();
    let expected = Rna::new("G").unwrap();
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn rna_complement_of_guanine_is_cytosine() {
    let input = "G";
    let output = Dna::new(input).unwrap().into_rna();
    let expected = Rna::new("C").unwrap();
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn rna_complement_of_thymine_is_adenine() {
    let input = "T";
    let output = Dna::new(input).unwrap().into_rna();
    let expected = Rna::new("A").unwrap();
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn rna_complement_of_adenine_is_uracil() {
    let input = "A";
    let output = Dna::new(input).unwrap().into_rna();
    let expected = Rna::new("U").unwrap();
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn rna_complement() {
    let input = "ACGTGGTCTTAA";
    let output = Dna::new(input).unwrap().into_rna();
    let expected = Rna::new("UGCACCAGAAUU").unwrap();
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn invalid_dna_input() {
    let input = "U";
    let output = Dna::new(input);
    let expected = Err(0);
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn invalid_dna_input_at_offset() {
    let input = "ACGTUXXCTTAA";
    let output = Dna::new(input);
    let expected = Err(4);
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn invalid_rna_input() {
    let input = "T";
    let output = Rna::new(input);
    let expected = Err(0);
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn invalid_rna_input_at_offset() {
    let input = "ACGTUXXCTTAA";
    let output = Rna::new(input);
    let expected = Err(3);
    assert_eq!(output, expected);
}
