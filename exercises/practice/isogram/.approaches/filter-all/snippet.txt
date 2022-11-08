pub fn check(candidate: &str) -> bool {
    let mut hs = std::collections::HashSet::new();
    candidate
        .bytes()
        .filter(|c| c.is_ascii_alphabetic())
        .map(|c| c.to_ascii_lowercase())
        .all(|c| hs.insert(c))
}
