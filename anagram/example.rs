use std::char;

pub fn anagrams_for(word: &str, inputs: &[&str]) -> ~[&str] {
    let lower_word = lowercase(word);
    let norm_word = alphagram(lower_word);
    inputs.iter()
        .filter(|&other| {
            let lower_other = lowercase(*other);
            lower_other != lower_word && norm_word == alphagram(lower_other)
        })
        .map(|s| *s)
        .collect()
}

fn lowercase(s: &str) -> ~str {
    s.chars().map(char::to_lowercase).collect::<~str>()
}

fn alphagram(s: &str) -> ~[char] {
    let mut chars: ~[char] = s.chars().collect();
    chars.sort();
    chars
}
