use anagram::*;
use std::collections::HashSet;

#[test]
fn no_matches() {
    let word = "diaper";
    let inputs = &["hello", "world", "zombies", "pants"];
    let output = anagrams_for(word, inputs);
    let expected = HashSet::from_iter([]);
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn detects_two_anagrams() {
    let word = "solemn";
    let inputs = &["lemons", "cherry", "melons"];
    let output = anagrams_for(word, inputs);
    let expected = HashSet::from_iter(["lemons", "melons"]);
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn does_not_detect_anagram_subsets() {
    let word = "good";
    let inputs = &["dog", "goody"];
    let output = anagrams_for(word, inputs);
    let expected = HashSet::from_iter([]);
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn detects_anagram() {
    let word = "listen";
    let inputs = &["enlists", "google", "inlets", "banana"];
    let output = anagrams_for(word, inputs);
    let expected = HashSet::from_iter(["inlets"]);
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn detects_three_anagrams() {
    let word = "allergy";
    let inputs = &[
        "gallery",
        "ballerina",
        "regally",
        "clergy",
        "largely",
        "leading",
    ];
    let output = anagrams_for(word, inputs);
    let expected = HashSet::from_iter(["gallery", "regally", "largely"]);
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn detects_multiple_anagrams_with_different_case() {
    let word = "nose";
    let inputs = &["Eons", "ONES"];
    let output = anagrams_for(word, inputs);
    let expected = HashSet::from_iter(["Eons", "ONES"]);
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn does_not_detect_non_anagrams_with_identical_checksum() {
    let word = "mass";
    let inputs = &["last"];
    let output = anagrams_for(word, inputs);
    let expected = HashSet::from_iter([]);
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn detects_anagrams_case_insensitively() {
    let word = "Orchestra";
    let inputs = &["cashregister", "Carthorse", "radishes"];
    let output = anagrams_for(word, inputs);
    let expected = HashSet::from_iter(["Carthorse"]);
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn detects_anagrams_using_case_insensitive_subject() {
    let word = "Orchestra";
    let inputs = &["cashregister", "carthorse", "radishes"];
    let output = anagrams_for(word, inputs);
    let expected = HashSet::from_iter(["carthorse"]);
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn detects_anagrams_using_case_insensitive_possible_matches() {
    let word = "orchestra";
    let inputs = &["cashregister", "Carthorse", "radishes"];
    let output = anagrams_for(word, inputs);
    let expected = HashSet::from_iter(["Carthorse"]);
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn does_not_detect_an_anagram_if_the_original_word_is_repeated() {
    let word = "go";
    let inputs = &["goGoGO"];
    let output = anagrams_for(word, inputs);
    let expected = HashSet::from_iter([]);
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn anagrams_must_use_all_letters_exactly_once() {
    let word = "tapper";
    let inputs = &["patter"];
    let output = anagrams_for(word, inputs);
    let expected = HashSet::from_iter([]);
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn words_are_not_anagrams_of_themselves() {
    let word = "BANANA";
    let inputs = &["BANANA"];
    let output = anagrams_for(word, inputs);
    let expected = HashSet::from_iter([]);
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn words_are_not_anagrams_of_themselves_even_if_letter_case_is_partially_different() {
    let word = "BANANA";
    let inputs = &["Banana"];
    let output = anagrams_for(word, inputs);
    let expected = HashSet::from_iter([]);
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn words_are_not_anagrams_of_themselves_even_if_letter_case_is_completely_different() {
    let word = "BANANA";
    let inputs = &["banana"];
    let output = anagrams_for(word, inputs);
    let expected = HashSet::from_iter([]);
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn words_other_than_themselves_can_be_anagrams() {
    let word = "LISTEN";
    let inputs = &["LISTEN", "Silent"];
    let output = anagrams_for(word, inputs);
    let expected = HashSet::from_iter(["Silent"]);
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn handles_case_of_greek_letters() {
    let word = "ΑΒΓ";
    let inputs = &["ΒΓΑ", "ΒΓΔ", "γβα", "αβγ"];
    let output = anagrams_for(word, inputs);
    let expected = HashSet::from_iter(["ΒΓΑ", "γβα"]);
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn different_characters_may_have_the_same_bytes() {
    let word = "a⬂";
    let inputs = &["€a"];
    let output = anagrams_for(word, inputs);
    let expected = HashSet::from_iter([]);
    assert_eq!(output, expected);
}
