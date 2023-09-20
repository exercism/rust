pub fn abbreviate(phrase: &str) -> String {
    phrase
        .split(|c: char| c.is_whitespace() || (c != '\'' && !c.is_alphanumeric()))
        .flat_map(split_camel)
        .filter_map(|word| word.chars().next())
        .collect::<String>()
        .to_uppercase()
}

fn split_camel(phrase: &str) -> Vec<String> {
    let chars: Vec<char> = phrase.chars().collect();
    let mut words: Vec<String> = Vec::new();
    let mut word_start: usize = 0;
    for (i, c) in chars.iter().enumerate() {
        if i == chars.len() - 1 || c.is_lowercase() && chars[i + 1].is_uppercase() {
            words.push(chars[word_start..=i].iter().cloned().collect());
            word_start = i + 1;
        }
    }
    words
}
