#![crate_name = "anagram_test"]
#![crate_type = "lib"]

mod anagram;

#[test]
#[ignore]
fn test_no_matches() {
    assert_eq!(
        anagram::anagrams_for("diaper", ["hello", "world", "zombies", "pants"]),
        vec!());
}

#[test]
#[ignore]
fn test_detect_simple_anagram() {
    assert_eq!(anagram::anagrams_for("ant", ["tan", "stand", "at"]), vec!("tan"));
}

#[test]
#[ignore]
fn test_does_not_confuse_different_duplicates() {
    assert_eq!(anagram::anagrams_for("galea", ["eagle"]), vec!());
}

#[test]
#[ignore]
fn test_eliminate_anagram_subsets() {
    assert_eq!(anagram::anagrams_for("good", ["dog", "goody"]), vec!());
}

#[test]
#[ignore]
fn test_detect_anagram() {
    assert_eq!(
        anagram::anagrams_for("listen",
                              ["enlists", "google", "inlets", "banana"]),
        vec!("inlets"))
}

#[test]
#[ignore]
fn test_multiple_anagrams() {
    assert_eq!(
        anagram::anagrams_for("allergy",
                              ["gallery", "ballerina", "regally",
                               "clergy", "largely", "leading"]),
        vec!("gallery", "regally", "largely"));
}

#[test]
#[ignore]
fn test_case_insensitive_anagrams() {
    assert_eq!(
        anagram::anagrams_for("Orchestra",
                              ["cashregister", "Carthorse", "radishes"]),
        vec!("Carthorse"));
}

#[test]
#[ignore]
fn test_unicode_anagrams() {
    // These words don't make sense, they're just greek letters cobbled together.
    assert_eq!(
        anagram::anagrams_for("ΑΒΓ",
                              ["ΒΓΑ", "ΒΓΔ", "γβα"]),
        vec!("ΒΓΑ", "γβα"));
}

#[test]
#[ignore]
fn test_misleading_unicode_anagrams() {
    // Despite what a human might think these words different letters, the input uses Greek A and B
    // while the list of potential anagrams uses Latin A and B.
    assert_eq!(
        anagram::anagrams_for("ΑΒΓ", ["ABΓ"]),
        vec!());
}

#[test]
#[ignore]
fn test_does_not_detect_a_word_as_its_own_anagram() {
    assert_eq!(anagram::anagrams_for("banana", ["banana"]), vec!());
}
