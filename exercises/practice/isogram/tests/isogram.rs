use isogram::check;

#[test]
fn empty_string() {
    assert_eq!(check(""), true, "An empty string should be an isogram.")
}

#[test]
#[ignore]
fn only_lower_case_characters() {
    assert_eq!(check("isogram"), true, "\"isogram\" should be an isogram.")
}

#[test]
#[ignore]
fn one_duplicated_character() {
    assert_eq!(
        check("eleven"),
        false,
        "\"eleven\" has more than one \'e\', therefore it is no isogram."
    )
}

#[test]
#[ignore]
fn longest_reported_english_isogram() {
    assert_eq!(
        check("subdermatoglyphic"),
        true,
        "\"subdermatoglyphic\" should be an isogram."
    )
}

#[test]
#[ignore]
fn one_duplicated_character_mixed_case() {
    assert_eq!(
        check("Alphabet"),
        false,
        "\"Alphabet\" has more than one \'a\', therefore it is no isogram."
    )
}

#[test]
#[ignore]
fn hypothetical_isogramic_word_with_hyphen() {
    assert_eq!(
        check("thumbscrew-japingly"),
        true,
        "\"thumbscrew-japingly\" should be an isogram."
    )
}

#[test]
#[ignore]
fn isogram_with_duplicated_hyphen() {
    assert_eq!(
        check("six-year-old"),
        true,
        "\"six-year-old\" should be an isogram."
    )
}

#[test]
#[ignore]
fn made_up_name_that_is_an_isogram() {
    assert_eq!(
        check("Emily Jung Schwartzkopf"),
        true,
        "\"Emily Jung Schwartzkopf\" should be an isogram."
    )
}

#[test]
#[ignore]
fn duplicated_character_in_the_middle() {
    assert_eq!(
        check("accentor"),
        false,
        "\"accentor\" has more than one \'c\', therefore it is no isogram."
    )
}
