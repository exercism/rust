#![feature(convert)]
#![feature(collections)]

use std::collections::HashMap;
use std::collections::hash_map::Entry;

pub fn word_count(input: &str) -> HashMap<String, u32> {
    let mut map: HashMap<String, u32> = HashMap::new();
    let norm = input.to_lowercase();
    let slice: &str = norm.as_ref();
    for word in slice.split(|c: char| !c.is_alphanumeric()).filter(|s| !s.is_empty()) {
        match map.entry(word.to_string()) {
            Entry::Vacant(entry) => { entry.insert(1); },
            Entry::Occupied(mut entry) => {
                *entry.get_mut() += 1;
            }
        };
    }
    map
}
