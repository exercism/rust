use std::collections::HashMap;
use once_cell::sync::Lazy;

static GLOBAL_LETTERS_SCORE: Lazy<HashMap<char, u64>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert('a', 1);
    m.insert('b', 3);
    m.insert('c', 3);
    m.insert('d', 2);
    m.insert('e', 1);
    m.insert('f', 4);
    m.insert('g', 2);
    m.insert('h', 4);
    m.insert('i', 1);
    m.insert('j', 8);
    m.insert('k', 5);
    m.insert('l', 1);
    m.insert('m', 3);
    m.insert('n', 1);
    m.insert('o', 1);
    m.insert('p', 3);
    m.insert('q', 10);
    m.insert('r', 1);
    m.insert('s', 1);
    m.insert('t', 1);
    m.insert('u', 1);
    m.insert('v', 4);
    m.insert('w', 4);
    m.insert('x', 8);
    m.insert('y', 4);
    m.insert('z', 10);
    m
});

/// Compute the Scrabble score for a word.
pub fn score(word: &str) -> u64 {
    word
    .to_ascii_lowercase()
    .chars()
    .filter(|c| c.is_ascii_alphabetic())
    .filter_map(|c| {
        GLOBAL_LETTERS_SCORE.get(&c)
    })
    .sum()
}
