use magazine_cutout::*;

#[test]
fn test_magazine_has_fewer_words_available_than_needed() {
    let magazine = "two times three is not four"
        .split_whitespace()
        .collect::<Vec<&str>>();
    let note = "two times two is four"
        .split_whitespace()
        .collect::<Vec<&str>>();
    assert!(!can_construct_note(&magazine, &note));
}

#[test]
#[ignore]
fn test_fn_returns_true_for_good_input() {
    let magazine = "The metro orchestra unveiled its new grand piano today. Its donor paraphrased Nathn Hale: \"I only regret that I have but one to give \"".split_whitespace().collect::<Vec<&str>>();
    let note = "give one grand today."
        .split_whitespace()
        .collect::<Vec<&str>>();
    assert!(can_construct_note(&magazine, &note));
}

#[test]
#[ignore]
fn test_fn_returns_false_for_bad_input() {
    let magazine = "I've got a lovely bunch of coconuts."
        .split_whitespace()
        .collect::<Vec<&str>>();
    let note = "I've got som coconuts"
        .split_whitespace()
        .collect::<Vec<&str>>();
    assert!(!can_construct_note(&magazine, &note));
}

#[test]
#[ignore]
fn test_case_sensitivity() {
    let magazine = "i've got some lovely coconuts"
        .split_whitespace()
        .collect::<Vec<&str>>();
    let note = "I've got some coconuts"
        .split_whitespace()
        .collect::<Vec<&str>>();
    assert!(!can_construct_note(&magazine, &note));

    let magazine = "I've got some lovely coconuts"
        .split_whitespace()
        .collect::<Vec<&str>>();
    let note = "i've got some coconuts"
        .split_whitespace()
        .collect::<Vec<&str>>();
    assert!(!can_construct_note(&magazine, &note));
}

#[test]
#[ignore]
fn test_magazine_has_more_words_available_than_needed() {
    let magazine = "Enough is enough when enough is enough"
        .split_whitespace()
        .collect::<Vec<&str>>();
    let note = "enough is enough".split_whitespace().collect::<Vec<&str>>();
    assert!(can_construct_note(&magazine, &note));
}

#[test]
#[ignore]
fn test_magazine_has_one_good_word_many_times_but_still_cant_construct() {
    let magazine = "A A A".split_whitespace().collect::<Vec<&str>>();
    let note = "A nice day".split_whitespace().collect::<Vec<&str>>();
    assert!(!can_construct_note(&magazine, &note));
}
