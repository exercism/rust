extern crate rna_transcription as dna;

#[test]
fn test_acid_equals_acid() {
    assert_eq!(dna::RibonucleicAcid::new("CGA"), dna::RibonucleicAcid::new("CGA"));
    assert_ne!(dna::RibonucleicAcid::new("CGA"), dna::RibonucleicAcid::new("AGC"));
}

#[test]
#[ignore]
fn test_transcribes_cytosine_guanine() {
    assert_eq!(Ok(dna::RibonucleicAcid::new("G")), dna::DeoxyribonucleicAcid::new("C").to_rna());
}

#[test]
#[ignore]
fn test_transcribes_guanine_cytosine() {
    assert_eq!(Ok(dna::RibonucleicAcid::new("C")), dna::DeoxyribonucleicAcid::new("G").to_rna());
}

#[test]
#[ignore]
fn test_transcribes_adenine_uracil() {
    assert_eq!(Ok(dna::RibonucleicAcid::new("U")), dna::DeoxyribonucleicAcid::new("A").to_rna());
}

#[test]
#[ignore]
fn test_transcribes_thymine_to_adenine() {
    assert_eq!(Ok(dna::RibonucleicAcid::new("A")), dna::DeoxyribonucleicAcid::new("T").to_rna());
}

#[test]
#[ignore]
fn test_transcribes_all_dna_to_rna() {
    assert_eq!(Ok(dna::RibonucleicAcid::new("UGCACCAGAAUU")), dna::DeoxyribonucleicAcid::new("ACGTGGTCTTAA").to_rna())
}

#[test]
#[ignore]
fn handles_invalid_input() {
    assert!(dna::DeoxyribonucleicAcid::new("U").to_rna().is_err());
}

#[test]
#[ignore]
fn handles_completely_invalid_input() {
    assert!(dna::DeoxyribonucleicAcid::new("XXX").to_rna().is_err());
}

#[test]
#[ignore]
fn handles_partially_invalid_input() {
    assert!(dna::DeoxyribonucleicAcid::new("ACGTXXXCTTAA").to_rna().is_err());
}
