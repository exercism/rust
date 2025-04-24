use std::collections::HashSet;

pub fn check(candidate: &str) -> bool {
    let cleaned_candidate:String = candidate.chars()
    .filter(|c| c.is_ascii_alphabetic())
    .map(|c| c.to_ascii_lowercase())
    .collect();

    let mut char_set:HashSet<char> = HashSet::new();

    for c in cleaned_candidate.chars() {
        if char_set.contains(&c) {
            return false
        }
        char_set.insert(c);
    }

    true
}
