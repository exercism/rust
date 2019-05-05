use scrabble_score::*;

#[test]
fn a_is_worth_one_point() {
    assert_eq!(1, score("a"));
}

#[test]
#[ignore]
fn scoring_is_case_insensitive() {
    assert_eq!(1, score("A"));
}

#[test]
#[ignore]
fn f_is_worth_four() {
    assert_eq!(4, score("f"));
}

#[test]
#[ignore]
fn two_one_point_letters_make_a_two_point_word() {
    assert_eq!(2, score("at"));
}

#[test]
#[ignore]
fn three_letter_word() {
    assert_eq!(12, score("zoo"));
}

#[test]
#[ignore]
fn medium_word() {
    assert_eq!(6, score("street"));
}

#[test]
#[ignore]
fn longer_words_with_valuable_letters() {
    assert_eq!(22, score("quirky"));
}

#[test]
#[ignore]
fn long_mixed_case_word() {
    assert_eq!(41, score("OxyphenButazone"));
}

#[test]
#[ignore]
fn non_english_scrabble_letters_do_not_score() {
    assert_eq!(8, score("pinata"), "'n' should score 1");
    assert_eq!(7, score("piñata"), "'ñ' should score 0");
}

#[test]
#[ignore]
fn empty_words_are_worth_zero() {
    assert_eq!(0, score(""));
}

#[test]
#[ignore]
fn all_letters_work() {
    assert_eq!(87, score("abcdefghijklmnopqrstuvwxyz"));
}

#[test]
#[ignore]
fn german_letters_do_not_score() {
    assert_eq!(7, "\"SS\" should score 2", score("STRASSE"));
    assert_eq!(5, score("STRAßE"), "'ß' should score 0");
}
