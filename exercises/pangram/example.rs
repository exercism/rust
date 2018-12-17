use std::collections::BTreeSet;
use std::iter::FromIterator;

pub fn is_pangram(sentence: &str) -> bool {
    sentence
        .to_lowercase()
        .chars()
        .filter(|c| c.is_alphabetic())
        .filter(|c| c.is_ascii())
        .collect::<BTreeSet<char>>()
        == english_letter_set()
}

fn english_letter_set() -> BTreeSet<char> {
    BTreeSet::from_iter(ENGLISH_ALPHABET.chars())
}

const ENGLISH_ALPHABET: &'static str = "abcdefghijklmnopqrstuvwxyz";
