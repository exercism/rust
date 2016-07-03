use std::collections::HashMap;

fn lowercase(word: &str) -> String {
    let mut lower = String::with_capacity(word.len());
    for c in word.chars() {
        for low in c.to_lowercase() {
            lower.push(low);
        }
    }
    lower
}

pub fn word_count(input: &str) -> HashMap<String, u32> {
    let mut map: HashMap<String, u32> = HashMap::new();
    let lower = lowercase(input);
    let slice: &str = lower.as_ref();
    for word in slice.split(|c: char| !c.is_alphanumeric()).filter(|s| !s.is_empty()) {
        *map.entry(word.to_string()).or_insert(0) += 1;
    }
    map
}
