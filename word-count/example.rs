#![allow(unstable)] // Entry, to_lowercase()

use std::collections::HashMap;
use std::collections::hash_map::Entry;

pub fn word_count(input: &str) -> HashMap<String, u32> {
    let mut map: HashMap<String, u32> = HashMap::new();
    let norm = input.chars().map(|&: c| c.to_lowercase()).collect::<String>();
    for word in norm.as_slice().split(|&: c: char| !c.is_alphanumeric()).filter(|s| !s.is_empty()) {
        match map.entry(word.to_string()) {
            Entry::Vacant(entry) => { entry.insert(1); },
            Entry::Occupied(mut entry) => {
                *entry.get_mut() += 1;
            }
        };
    }
    map
}
