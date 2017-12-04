extern crate rna_transcription as dna;

#[test]
fn test_acid_equals_acid() {
    assert_eq!(dna::RNA::new("CGA"), dna::RNA::new("CGA"));
    assert_ne!(dna::RNA::new("CGA"), dna::RNA::new("AGC"));
}

#[test]
#[ignore]
fn test_transcribes_cytosine_guanine() {
    assert_eq!(Ok(dna::RNA::new("G")), dna::DNA::new("C").to_rna());
}

#[test]
#[ignore]
fn test_transcribes_guanine_cytosine() {
    assert_eq!(Ok(dna::RNA::new("C")), dna::DNA::new("G").to_rna());
}

#[test]
#[ignore]
fn test_transcribes_adenine_uracil() {
    assert_eq!(Ok(dna::RNA::new("U")), dna::DNA::new("A").to_rna());
}

#[test]
#[ignore]
fn test_transcribes_thymine_to_adenine() {
    assert_eq!(Ok(dna::RNA::new("A")), dna::DNA::new("T").to_rna());
}

#[test]
#[ignore]
fn test_transcribes_all_dna_to_rna() {
    assert_eq!(Ok(dna::RNA::new("UGCACCAGAAUU")), dna::DNA::new("ACGTGGTCTTAA").to_rna())
}

#[test]
#[ignore]
fn handles_invalid_input() {
    assert!(dna::DNA::new("U").to_rna().is_err());
}

#[test]
#[ignore]
fn handles_completely_invalid_input() {
    assert!(dna::DNA::new("XXX").to_rna().is_err());
}

#[test]
#[ignore]
fn handles_partially_invalid_input() {
    assert!(dna::DNA::new("ACGTXXXCTTAA").to_rna().is_err());
}
