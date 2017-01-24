extern crate scrabble_score;

use scrabble_score::*;

#[test]
fn a_is_worth_one_point() {
    assert_eq!(score("a"), 1);
}

#[test]
#[ignore]
fn scoring_is_case_insensitive() {
    assert_eq!(score("A"), 1);
}

#[test]
#[ignore]
fn f_is_worth_four() {
    assert_eq!(score("f"), 4);
}

#[test]
#[ignore]
fn two_one_point_letters_make_a_two_point_word() {
    assert_eq!(score("at"), 2);
}

#[test]
#[ignore]
fn three_letter_word() {
    assert_eq!(score("zoo"), 12);
}

#[test]
#[ignore]
fn medium_word() {
    assert_eq!(score("street"), 6);
}

#[test]
#[ignore]
fn longer_words_with_valuable_letters() {
    assert_eq!(score("quirky"), 22);
}

#[test]
#[ignore]
fn long_mixed_case_word() {
    assert_eq!(score("OxyphenButazone"), 41);
}

#[test]
#[ignore]
fn non_english_scrabble_letters_do_not_score() {
    assert_eq!(score("pinata"), 8);
    assert_eq!(score("piñata"), 7);
}

#[test]
#[ignore]
fn empty_words_are_worth_zero() {
    assert_eq!(score(""), 0);
}

#[test]
#[ignore]
fn all_letters_work() {
    assert_eq!(score("abcdefghijklmnopqrstuvwxyz"), 87);
}
