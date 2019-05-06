use rna_transcription as dna;

#[test]
fn test_valid_dna_input() {
    assert!(dna::DNA::new("GCTA").is_ok());
}

#[test]
#[ignore]
fn test_valid_rna_input() {
    assert!(dna::RNA::new("CGAU").is_ok());
}

#[test]
#[ignore]
fn test_invalid_dna_input() {
    // Invalid character
    assert_eq!(dna::DNA::new("X").err(), Some(0));
    // Valid nucleotide, but invalid in context
    assert_eq!(dna::DNA::new("U").err(), Some(0));
    // Longer string with contained errors
    assert_eq!(dna::DNA::new("ACGTUXXCTTAA").err(), Some(4));
}

#[test]
#[ignore]
fn test_invalid_rna_input() {
    // Invalid character
    assert!(dna::RNA::new("X").is_err());
    // Valid nucleotide, but invalid in context
    assert!(dna::RNA::new("T").is_err());
    // Longer string with contained errors
    assert!(dna::RNA::new("ACGUTTXCUUAA").is_err());
}

#[test]
#[ignore]
fn test_acid_equals_acid() {
    assert_eq!(dna::DNA::new("CGA").unwrap(), dna::DNA::new("CGA").unwrap());
    assert_ne!(dna::DNA::new("CGA").unwrap(), dna::DNA::new("AGC").unwrap());
    assert_eq!(dna::RNA::new("CGA").unwrap(), dna::RNA::new("CGA").unwrap());
    assert_ne!(dna::RNA::new("CGA").unwrap(), dna::RNA::new("AGC").unwrap());
}

#[test]
#[ignore]
fn test_transcribes_cytosine_guanine() {
    assert_eq!(
        dna::RNA::new("G").unwrap(),
        dna::DNA::new("C").unwrap().into_rna()
    );
}

#[test]
#[ignore]
fn test_transcribes_guanine_cytosine() {
    assert_eq!(
        dna::RNA::new("C").unwrap(),
        dna::DNA::new("G").unwrap().into_rna()
    );
}

#[test]
#[ignore]
fn test_transcribes_adenine_uracil() {
    assert_eq!(
        dna::RNA::new("U").unwrap(),
        dna::DNA::new("A").unwrap().into_rna()
    );
}

#[test]
#[ignore]
fn test_transcribes_thymine_to_adenine() {
    assert_eq!(
        dna::RNA::new("A").unwrap(),
        dna::DNA::new("T").unwrap().into_rna()
    );
}

#[test]
#[ignore]
fn test_transcribes_all_dna_to_rna() {
    assert_eq!(
        dna::RNA::new("UGCACCAGAAUU").unwrap(),
        dna::DNA::new("ACGTGGTCTTAA").unwrap().into_rna()
    )
}
