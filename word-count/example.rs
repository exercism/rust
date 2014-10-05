use std::collections::HashMap;
use std::collections::hashmap::{Occupied, Vacant};

pub fn word_count(input: &str) -> HashMap<String, uint> {
    let mut map: HashMap<String, uint> = HashMap::new();
    let norm = input.chars().map(|c| c.to_lowercase()).collect::<String>();
    for word in norm.as_slice().split(|c: char| !c.is_alphanumeric()).filter(|s| !s.is_empty()) {
        match map.entry(word.to_string()) {
            Vacant(entry) => entry.set(1),
            Occupied(mut entry) => {
                *entry.get_mut() += 1;
                entry.into_mut()
            }
        };
    }
    map
}
