use pangram::*;

#[test]
fn empty_strings_are_not_pangrams() {
    let sentence = "";
    assert!(!is_pangram(&sentence));
}

#[test]
#[ignore]
fn classic_pangram_is_a_pangram() {
    let sentence = "the quick brown fox jumps over the lazy dog";
    assert!(is_pangram(&sentence));
}

#[test]
#[ignore]
fn pangrams_must_have_all_letters() {
    let sentence = "a quick movement of the enemy will jeopardize five gunboats";
    assert!(!is_pangram(&sentence));
}

#[test]
#[ignore]
fn pangrams_must_have_all_letters_two() {
    let sentence = "the quick brown fish jumps over the lazy dog";
    assert!(!is_pangram(&sentence));
}

#[test]
#[ignore]
fn pangrams_must_include_z() {
    let sentence = "the quick brown fox jumps over the lay dog";
    assert!(!is_pangram(&sentence));
}

#[test]
#[ignore]
fn underscores_do_not_affect_pangrams() {
    let sentence = "the_quick_brown_fox_jumps_over_the_lazy_dog";
    assert!(is_pangram(&sentence));
}

#[test]
#[ignore]
fn numbers_do_not_affect_pangrams() {
    let sentence = "the 1 quick brown fox jumps over the 2 lazy dogs";
    assert!(is_pangram(&sentence));
}

#[test]
#[ignore]
fn numbers_can_not_replace_letters() {
    let sentence = "7h3 qu1ck brown fox jumps ov3r 7h3 lazy dog";
    assert!(!is_pangram(&sentence));
}

#[test]
#[ignore]
fn capitals_and_punctuation_can_be_in_pangrams() {
    let sentence = "\"Five quacking Zephyrs jolt my wax bed.\"";
    assert!(is_pangram(&sentence));
}

#[test]
#[ignore]
fn non_ascii_characters_can_be_in_pangrams() {
    let sentence = "Victor jagt zwölf Boxkämpfer quer über den großen Sylter Deich.";
    assert!(is_pangram(&sentence));
}
