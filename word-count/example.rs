extern crate collections;

use collections::hashmap::HashMap;

pub fn word_count(input: &str) -> HashMap<~str, uint> {
    let mut map: HashMap<~str, uint> = HashMap::new();
    let norm: ~str = input.chars().map(|c| c.to_lowercase()).collect();
    for word in norm.split(|c: char| !c.is_alphanumeric()).filter(|s| !s.is_empty()) {
        map.insert_or_update_with(word.to_owned(), 1, |_, count| *count += 1);
    }
    map
}
