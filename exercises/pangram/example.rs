use std::collections::BTreeSet;

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
    ENGLISH_ALPHABET.chars().collect()
}

const ENGLISH_ALPHABET: &str = "abcdefghijklmnopqrstuvwxyz";
