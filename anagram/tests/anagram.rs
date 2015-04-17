extern crate anagram;

#[test]
fn test_no_matches() {
    let inputs = ["hello", "world", "zombies", "pants"];
    let outputs: Vec<&str> = vec![];
    assert_eq!(anagram::anagrams_for("diaper", &inputs), outputs);
}

#[test]
fn test_detect_simple_anagram() {
    let inputs = ["tan", "stand", "at"];
    let outputs: Vec<&str> = vec!["tan"];
    assert_eq!(anagram::anagrams_for("ant", &inputs), outputs);
}

#[test]
fn test_does_not_confuse_different_duplicates() {
    let inputs = ["eagle"];
    let outputs: Vec<&str> = vec![];
    assert_eq!(anagram::anagrams_for("galea", &inputs), outputs);
}

#[test]
fn test_eliminate_anagram_subsets() {
    let inputs = ["dog", "goody"];
    let outputs: Vec<&str> = vec![];
    assert_eq!(anagram::anagrams_for("good", &inputs), outputs);
}

#[test]
fn test_detect_anagram() {
    let inputs = ["enlists", "google", "inlets", "banana"];
    let outputs: Vec<&str> = vec!["inlets"];
    assert_eq!(anagram::anagrams_for("listen", &inputs), outputs);
}

#[test]
fn test_multiple_anagrams() {
    let inputs = ["gallery", "ballerina", "regally", "clergy", "largely", "leading"];
    let outputs: Vec<&str> = vec!["gallery", "regally", "largely"];
    assert_eq!(anagram::anagrams_for("allergy", &inputs), outputs); 
}

#[test]
fn test_case_insensitive_anagrams() {
    let inputs = ["cashregister", "Carthorse", "radishes"];
    let outputs: Vec<&str> = vec!["Carthorse"];
    assert_eq!(anagram::anagrams_for("Orchestra", &inputs), outputs); 
}

#[test]
fn test_unicode_anagrams() {
    // These words don't make sense, they're just greek letters cobbled together.
    let inputs = ["ΒΓΑ", "ΒΓΔ", "γβα"];
    let outputs: Vec<&str> = vec!["ΒΓΑ", "γβα"];
    assert_eq!(anagram::anagrams_for("ΑΒΓ", &inputs), outputs);
}

#[test]
fn test_misleading_unicode_anagrams() {
    // Despite what a human might think these words different letters, the input uses Greek A and B
    // while the list of potential anagrams uses Latin A and B.
    let inputs = ["ABΓ"];
    let outputs: Vec<&str> = vec![];
    assert_eq!(anagram::anagrams_for("ΑΒΓ", &inputs), outputs);
}

#[test]
fn test_does_not_detect_a_word_as_its_own_anagram() {
    let inputs = ["banana"];
    let outputs: Vec<&str> = vec![];
    assert_eq!(anagram::anagrams_for("banana", &inputs), outputs);
}
