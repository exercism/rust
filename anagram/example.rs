#![allow(unstable)] // For to_lowercase

pub fn anagrams_for<'a>(word: &str, inputs: &[&'a str]) -> Vec<&'a str> {
    let lower_word: Vec<char> = word.chars().map(|c| c.to_lowercase()).collect();
    let mut norm_word = lower_word.clone();
    norm_word.sort();
    inputs.iter()
        .filter(|other| {
            let lower_other: Vec<char> = other.chars().map(|c| c.to_lowercase()).collect();
            let mut norm_other = lower_other.clone();
            norm_other.sort();
            lower_other != lower_word && norm_other == norm_word
        })
        .map(|s| *s)
        .collect()
}
