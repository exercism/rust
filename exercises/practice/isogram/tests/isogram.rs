use isogram::check;

#[test]
fn empty_string() {
    assert!(check(""), "An empty string should be an isogram.")
}

#[test]
#[ignore]
fn only_lower_case_characters() {
    assert!(check("isogram"), "\"isogram\" should be an isogram.")
}

#[test]
#[ignore]
fn one_duplicated_character() {
    assert!(
        !check("eleven"),
        "\"eleven\" has more than one \'e\', therefore it is no isogram."
    )
}

#[test]
#[ignore]
fn longest_reported_english_isogram() {
    assert!(
        check("subdermatoglyphic"),
        "\"subdermatoglyphic\" should be an isogram."
    )
}

#[test]
#[ignore]
fn one_duplicated_character_mixed_case() {
    assert!(
        !check("Alphabet"),
        "\"Alphabet\" has more than one \'a\', therefore it is no isogram."
    )
}

#[test]
#[ignore]
fn hypothetical_isogramic_word_with_hyphen() {
    assert!(
        check("thumbscrew-japingly"),
        "\"thumbscrew-japingly\" should be an isogram."
    )
}

#[test]
#[ignore]
fn isogram_with_duplicated_hyphen() {
    assert!(
        check("six-year-old"),
        "\"six-year-old\" should be an isogram."
    )
}

#[test]
#[ignore]
fn made_up_name_that_is_an_isogram() {
    assert!(
        check("Emily Jung Schwartzkopf"),
        "\"Emily Jung Schwartzkopf\" should be an isogram."
    )
}

#[test]
#[ignore]
fn duplicated_character_in_the_middle() {
    assert!(
        !check("accentor"),
        "\"accentor\" has more than one \'c\', therefore it is no isogram."
    )
}
