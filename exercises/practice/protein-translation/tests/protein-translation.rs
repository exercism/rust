use protein_translation::*;

#[test]
fn empty_rna_sequence_results_in_no_proteins() {
    assert_eq!(translate(""), Some(vec![]),);
}

#[test]
#[ignore]
fn methionine_rna_sequence() {
    assert_eq!(translate("AUG"), Some(vec!["methionine"]),);
}

#[test]
#[ignore]
fn phenylalanine_rna_sequence_1() {
    assert_eq!(translate("UUU"), Some(vec!["phenylalanine"]),);
}

#[test]
#[ignore]
fn phenylalanine_rna_sequence_2() {
    assert_eq!(translate("UUC"), Some(vec!["phenylalanine"]),);
}

#[test]
#[ignore]
fn leucine_rna_sequence_1() {
    assert_eq!(translate("UUA"), Some(vec!["leucine"]),);
}

#[test]
#[ignore]
fn leucine_rna_sequence_2() {
    assert_eq!(translate("UUG"), Some(vec!["leucine"]),);
}

#[test]
#[ignore]
fn serine_rna_sequence_1() {
    assert_eq!(translate("UCU"), Some(vec!["serine"]),);
}

#[test]
#[ignore]
fn serine_rna_sequence_2() {
    assert_eq!(translate("UCC"), Some(vec!["serine"]),);
}

#[test]
#[ignore]
fn serine_rna_sequence_3() {
    assert_eq!(translate("UCA"), Some(vec!["serine"]),);
}

#[test]
#[ignore]
fn serine_rna_sequence_4() {
    assert_eq!(translate("UCG"), Some(vec!["serine"]),);
}

#[test]
#[ignore]
fn tyrosine_rna_sequence_1() {
    assert_eq!(translate("UAU"), Some(vec!["tyrosine"]),);
}

#[test]
#[ignore]
fn tyrosine_rna_sequence_2() {
    assert_eq!(translate("UAC"), Some(vec!["tyrosine"]),);
}

#[test]
#[ignore]
fn cysteine_rna_sequence_1() {
    assert_eq!(translate("UGU"), Some(vec!["cysteine"]),);
}

#[test]
#[ignore]
fn cysteine_rna_sequence_2() {
    assert_eq!(translate("UGC"), Some(vec!["cysteine"]),);
}

#[test]
#[ignore]
fn tryptophan_rna_sequence() {
    assert_eq!(translate("UGG"), Some(vec!["tryptophan"]),);
}

#[test]
#[ignore]
fn stop_codon_rna_sequence_1() {
    assert_eq!(translate("UAA"), Some(vec![]),);
}

#[test]
#[ignore]
fn stop_codon_rna_sequence_2() {
    assert_eq!(translate("UAG"), Some(vec![]),);
}

#[test]
#[ignore]
fn stop_codon_rna_sequence_3() {
    assert_eq!(translate("UGA"), Some(vec![]),);
}

#[test]
#[ignore]
fn sequence_of_two_protein_codons_translates_into_proteins() {
    assert_eq!(
        translate("UUUUUU"),
        Some(vec!["phenylalanine", "phenylalanine"]),
    );
}

#[test]
#[ignore]
fn sequence_of_two_different_protein_codons_translates_into_proteins() {
    assert_eq!(translate("UUAUUG"), Some(vec!["leucine", "leucine"]),);
}

#[test]
#[ignore]
fn translate_rna_strand_into_correct_protein_list() {
    assert_eq!(
        translate("AUGUUUUGG"),
        Some(vec!["methionine", "phenylalanine", "tryptophan"]),
    );
}

#[test]
#[ignore]
fn translation_stops_if_stop_codon_at_beginning_of_sequence() {
    assert_eq!(translate("UAGUGG"), Some(vec![]),);
}

#[test]
#[ignore]
fn translation_stops_if_stop_codon_at_end_of_two_codon_sequence() {
    assert_eq!(translate("UGGUAG"), Some(vec!["tryptophan"]),);
}

#[test]
#[ignore]
fn translation_stops_if_stop_codon_at_end_of_three_codon_sequence() {
    assert_eq!(
        translate("AUGUUUUAA"),
        Some(vec!["methionine", "phenylalanine"]),
    );
}

#[test]
#[ignore]
fn translation_stops_if_stop_codon_in_middle_of_three_codon_sequence() {
    assert_eq!(translate("UGGUAGUGG"), Some(vec!["tryptophan"]),);
}

#[test]
#[ignore]
fn translation_stops_if_stop_codon_in_middle_of_six_codon_sequence() {
    assert_eq!(
        translate("UGGUGUUAUUAAUGGUUU"),
        Some(vec!["tryptophan", "cysteine", "tyrosine"]),
    );
}

#[test]
#[ignore]
fn sequence_of_two_non_stop_codons_does_not_translate_to_a_stop_codon() {
    assert_eq!(translate("AUGAUG"), Some(vec!["methionine", "methionine"]),);
}

#[test]
#[ignore]
fn unknown_amino_acids_not_part_of_a_codon_can_t_translate() {
    assert_eq!(translate("XYZ"), None,);
}

#[test]
#[ignore]
fn incomplete_rna_sequence_can_t_translate() {
    assert_eq!(translate("AUGU"), None,);
}

#[test]
#[ignore]
fn incomplete_rna_sequence_can_translate_if_valid_until_a_stop_codon() {
    assert_eq!(
        translate("UUCUUCUAAUGGU"),
        Some(vec!["phenylalanine", "phenylalanine"]),
    );
}
