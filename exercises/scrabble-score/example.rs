pub fn score(word: &str) -> u16 {
    let mut score: u16 = 0;
    let mut previous_char_value: u16 = 0;

    // normalise the word to lowercase, for case-insensitivity, also, for easier iteration,
    // reduce the multi-character value multipliers :double and :triple to a single char.
    let tokenised_word = word.to_lowercase().replace(":double", "2").replace(":triple", "3");

    for a_char in tokenised_word.chars() {
        let value = char_value(a_char, previous_char_value);
        score += value;
        previous_char_value = value;
    }

    score
}

/// Calculates the value of the current character.
///
/// The previous characters' value is required in case the current
/// token is a multiplier, which affects the score of the previous char.
fn char_value(the_char: char, prev_value: u16) -> u16 {
    match the_char {
        'a'|'e'|'i'|'o'|'u'|'l'|'n'|'r'|'s'|'t' => 1,
        'd'|'g' => 2,
        'b'|'c'|'m'|'p' => 3,
        'f'|'h'|'v'|'w'|'y' => 4,
        'k' => 5,
        'j'|'x' => 8,
        'q'|'z' => 10,
        '2' =>   prev_value, // repeat the previous value, effectively doubling the previous value
        '3' => 2*prev_value, // repeat the previous value twice, effectively multiplying it by 3x
        _   => 0, // 'funny' characters, such as accented ones or foreign alphabets, count for 0
    }
}
