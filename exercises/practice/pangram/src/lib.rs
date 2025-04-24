use std::collections::HashSet;

/// Determine whether a sentence is a pangram.
pub fn is_pangram(sentence: &str) -> bool {
    let mut set: HashSet<char> = HashSet::new();

    sentence
    .chars()
    .filter(|c| c.is_ascii_alphabetic())
    .map(|c| c.to_ascii_lowercase())
    .for_each(|c| {
        set.insert(c);
    });

    set.len() == 26
}
