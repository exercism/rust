use hamming;

fn process_distance_case(strand_pair: [&str; 2], expected_distance: Option<usize>) {
    assert_eq!(
        hamming::hamming_distance(strand_pair[0], strand_pair[1]),
        expected_distance
    );
}

#[test]
fn test_empty_strands() {
    process_distance_case(["", ""], Some(0));
}

#[test]
#[ignore]
fn test_no_difference_between_identical_strands() {
    process_distance_case(["GGACTGA", "GGACTGA"], Some(0));
}

#[test]
#[ignore]
fn test_complete_hamming_distance_in_small_strand() {
    process_distance_case(["ACT", "GGA"], Some(3));
}

#[test]
#[ignore]
fn test_small_hamming_distance_in_the_middle_somewhere() {
    process_distance_case(["GGACG", "GGTCG"], Some(1));
}

#[test]
#[ignore]
fn test_larger_distance() {
    process_distance_case(["ACCAGGG", "ACTATGG"], Some(2));
}

#[test]
#[ignore]
fn test_first_string_is_longer() {
    process_distance_case(["AAA", "AA"], None);
}

#[test]
#[ignore]
fn test_second_string_is_longer() {
    process_distance_case(["A", "AA"], None);
}

#[test]
#[ignore]
/// non-unique character in first strand
fn test_nonunique_character_in_first_strand() {
    process_distance_case(["AAG", "AAA"], Some(1));
}

#[test]
#[ignore]
/// identical strands
fn test_identical_strands() {
    process_distance_case(["A", "A"], Some(0));
}

#[test]
#[ignore]
/// complete distance in small strands
fn test_complete_distance_in_small_strands() {
    process_distance_case(["AG", "CT"], Some(2));
}

#[test]
#[ignore]
/// disallow first strand longer
fn test_disallow_first_strand_longer() {
    process_distance_case(["AATG", "AAA"], None);
}

#[test]
#[ignore]
/// large distance
fn test_large_distance() {
    process_distance_case(["GATACA", "GCATAA"], Some(4));
}

#[test]
#[ignore]
/// long identical strands
fn test_long_identical_strands() {
    process_distance_case(["GGACTGA", "GGACTGA"], Some(0));
}

#[test]
#[ignore]
/// complete distance in single nucleotide strands
fn test_complete_distance_in_single_nucleotide_strands() {
    process_distance_case(["A", "G"], Some(1));
}

#[test]
#[ignore]
/// small distance
fn test_small_distance() {
    process_distance_case(["GGACG", "GGTCG"], Some(1));
}

#[test]
#[ignore]
/// non-unique character in second strand
fn test_nonunique_character_in_second_strand() {
    process_distance_case(["AAA", "AAG"], Some(1));
}

#[test]
#[ignore]
/// small distance in long strands
fn test_small_distance_in_long_strands() {
    process_distance_case(["ACCAGGG", "ACTATGG"], Some(2));
}

#[test]
#[ignore]
/// disallow second strand longer
fn test_disallow_second_strand_longer() {
    process_distance_case(["ATA", "AGTG"], None);
}

#[test]
#[ignore]
/// small distance in small strands
fn test_small_distance_in_small_strands() {
    process_distance_case(["AT", "CT"], Some(1));
}

#[test]
#[ignore]
/// large distance in off-by-one strand
fn test_large_distance_in_offbyone_strand() {
    process_distance_case(["GGACGGATTCTG", "AGGACGGATTCT"], Some(9));
}

#[test]
#[ignore]
/// same nucleotides in different positions
fn test_same_nucleotides_in_different_positions() {
    process_distance_case(["TAG", "GAT"], Some(2));
}
