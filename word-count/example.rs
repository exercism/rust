use std::collections::HashMap;

pub fn word_count(input: &str) -> HashMap<String, uint> {
    let mut map: HashMap<String, uint> = HashMap::new();
    let norm = input.chars().map(|c| c.to_lowercase()).collect::<String>();
    for word in norm.as_slice().split(|c: char| !c.is_alphanumeric()).filter(|s| !s.is_empty()) {
        map.insert_or_update_with(word.to_string(), 1, |_, count| *count += 1);
    }
    map
}
