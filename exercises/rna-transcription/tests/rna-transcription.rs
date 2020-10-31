use rna_transcription as dna;

#[test]
fn test_valid_dna_input() {
    assert!(dna::Dna::new("GCTA").is_ok());
}

#[test]
#[ignore]
fn test_valid_rna_input() {
    assert!(dna::Rna::new("CGAU").is_ok());
}

#[test]
#[ignore]
fn test_invalid_dna_input() {
    // Invalid character
    assert_eq!(dna::Dna::new("X").err(), Some(0));
    // Valid nucleotide, but invalid in context
    assert_eq!(dna::Dna::new("U").err(), Some(0));
    // Longer string with contained errors
    assert_eq!(dna::Dna::new("ACGTUXXCTTAA").err(), Some(4));
}

#[test]
#[ignore]
fn test_invalid_rna_input() {
    // Invalid character
    assert_eq!(dna::Rna::new("X").unwrap_err(), 0);
    // Valid nucleotide, but invalid in context
    assert_eq!(dna::Rna::new("T").unwrap_err(), 0);
    // Longer string with contained errors
    assert_eq!(dna::Rna::new("ACGUTTXCUUAA").unwrap_err(), 4);
}

#[test]
#[ignore]
fn test_acid_equals_acid() {
    assert_eq!(dna::Dna::new("CGA").unwrap(), dna::Dna::new("CGA").unwrap());
    assert_ne!(dna::Dna::new("CGA").unwrap(), dna::Dna::new("AGC").unwrap());
    assert_eq!(dna::Rna::new("CGA").unwrap(), dna::Rna::new("CGA").unwrap());
    assert_ne!(dna::Rna::new("CGA").unwrap(), dna::Rna::new("AGC").unwrap());
}

#[test]
#[ignore]
fn test_transcribes_cytosine_guanine() {
    assert_eq!(
        dna::Rna::new("G").unwrap(),
        dna::Dna::new("C").unwrap().into_rna()
    );
}

#[test]
#[ignore]
fn test_transcribes_guanine_cytosine() {
    assert_eq!(
        dna::Rna::new("C").unwrap(),
        dna::Dna::new("G").unwrap().into_rna()
    );
}

#[test]
#[ignore]
fn test_transcribes_adenine_uracil() {
    assert_eq!(
        dna::Rna::new("U").unwrap(),
        dna::Dna::new("A").unwrap().into_rna()
    );
}

#[test]
#[ignore]
fn test_transcribes_thymine_to_adenine() {
    assert_eq!(
        dna::Rna::new("A").unwrap(),
        dna::Dna::new("T").unwrap().into_rna()
    );
}

#[test]
#[ignore]
fn test_transcribes_all_dna_to_rna() {
    assert_eq!(
        dna::Rna::new("UGCACCAGAAUU").unwrap(),
        dna::Dna::new("ACGTGGTCTTAA").unwrap().into_rna()
    )
}
