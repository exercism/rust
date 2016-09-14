///
/// Techies love their TLA (Three Letter Acronyms)!
///
/// Help generate some jargon by writing a program that converts a long name
/// like Portable Network Graphics to its acronym (PNG).
#[allow(unused_variables)]
pub fn abbreviate(phrase: &str) -> String {
    phrase.split(|c: char| c.is_whitespace() || !c.is_alphanumeric())
          .flat_map(|word| split_camel(word))
          .filter(|word| !word.is_empty())
          .map(|word| word.split_at(1).0.to_uppercase())
          .collect::<String>()
}

fn split_camel(phrase: &str) -> Vec<String> {
    let chars: Vec<char> = phrase.chars().collect();
    let mut words: Vec<String> = Vec::new();
    let mut prev_ind: usize = 0;
    for (i, c) in chars.iter().enumerate() {
        if i > 0 && i == chars.len() - 1 || i > 0 && c.is_lowercase() && chars[i + 1].is_uppercase() {
            words.push(chars[prev_ind..i + 1].iter().cloned().collect());
            prev_ind = i + 1;
        }
    }
    words
}
