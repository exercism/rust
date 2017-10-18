extern crate isogram;

use isogram::check;

#[test]
fn empty_string() {
    assert_eq!(check(""), true, "An empty string is a isogram.")
}

#[test]
fn only_lower_case_characters() {
    assert_eq!(check("isogram"), true, "\"isogram\" is a isogram.")
}

#[test]
fn one_duplicated_character() {
    assert_eq!(check("eleven"),
               false,
               "\"eleven\" has duplicated \'e\' isogram.")
}

#[test]
fn longest_reported_english_isogram() {
    assert_eq!(check("subdermatoglyphic"),
               true,
               "\"subdermatoglyphic\" is a isogram.")
}

#[test]
fn one_duplicated_character_mixed_case() {
    assert_eq!(check("Alphabet"),
               false,
               "\"Alphabet\" has duplicated \'a\' isogram.")
}

#[test]
fn hypothetical_isogramic_word_with_hyphen() {
    assert_eq!(check("thumbscrew-japingly"),
               true,
               "\"thumbscrew-japingly\" is a isogram.")
}

#[test]
fn isogram_with_duplicated_hyphen() {
    assert_eq!(check("six-year-old"),
               true,
               "\"six-year-old\" is a isogram.")
}

#[test]
fn made_up_name_that_is_an_isogram() {
    assert_eq!(check("Emily Jung Schwartzkopf"),
               true,
               "\"Emily Jung Schwartzkopf\" is a isogram.")
}

#[test]
fn duplicated_character_in_the_middle() {
    assert_eq!(check("accentor"),
               false,
               "\"accentor\" has duplicated \'c\' isogram.")
}
