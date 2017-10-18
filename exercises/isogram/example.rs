pub fn check(word: &str) -> bool {

    // Filter all non-Alphabetic character out and collect them in a new String
    let normalized_string: String = word.to_lowercase().chars().filter(|c| c.is_alphabetic()).collect();

    // Find the char element from back and front and compare the index
    let closure = |x: char, word: &str| word.find(x).unwrap() == word.rfind(x).unwrap();

    // Length should be the same if it is a isogram
    normalized_string.len() == normalized_string.chars().filter(|&x| closure(x, normalized_string.as_str())).count()
}
