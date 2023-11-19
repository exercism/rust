#[test]
fn an_empty_string() {
    let input = "";
    let output = reverse_string::reverse(input);
    let expected = "";
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn a_word() {
    let input = "robot";
    let output = reverse_string::reverse(input);
    let expected = "tobor";
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn a_capitalized_word() {
    let input = "Ramen";
    let output = reverse_string::reverse(input);
    let expected = "nemaR";
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn a_sentence_with_punctuation() {
    let input = "I'm hungry!";
    let output = reverse_string::reverse(input);
    let expected = "!yrgnuh m'I";
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn a_palindrome() {
    let input = "racecar";
    let output = reverse_string::reverse(input);
    let expected = "racecar";
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn an_even_sized_word() {
    let input = "drawer";
    let output = reverse_string::reverse(input);
    let expected = "reward";
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn wide_characters() {
    let input = "子猫";
    let output = reverse_string::reverse(input);
    let expected = "猫子";
    assert_eq!(output, expected);
}

#[test]
#[ignore]
#[cfg(feature = "grapheme")]
fn grapheme_clusters() {
    let input = "uüu";
    let output = reverse_string::reverse(input);
    let expected = "uüu";
    assert_eq!(output, expected);
}
