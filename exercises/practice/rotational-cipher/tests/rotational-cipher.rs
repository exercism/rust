use rotational_cipher as cipher;

#[test]
fn rotate_a_by_0_same_output_as_input() {
    let text = "a";
    let shift_key = 0;
    let output = cipher::rotate(text, shift_key);
    let expected = "a";
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn rotate_a_by_1() {
    let text = "a";
    let shift_key = 1;
    let output = cipher::rotate(text, shift_key);
    let expected = "b";
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn rotate_a_by_26_same_output_as_input() {
    let text = "a";
    let shift_key = 26;
    let output = cipher::rotate(text, shift_key);
    let expected = "a";
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn rotate_m_by_13() {
    let text = "m";
    let shift_key = 13;
    let output = cipher::rotate(text, shift_key);
    let expected = "z";
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn rotate_n_by_13_with_wrap_around_alphabet() {
    let text = "n";
    let shift_key = 13;
    let output = cipher::rotate(text, shift_key);
    let expected = "a";
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn rotate_capital_letters() {
    let text = "OMG";
    let shift_key = 5;
    let output = cipher::rotate(text, shift_key);
    let expected = "TRL";
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn rotate_spaces() {
    let text = "O M G";
    let shift_key = 5;
    let output = cipher::rotate(text, shift_key);
    let expected = "T R L";
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn rotate_numbers() {
    let text = "Testing 1 2 3 testing";
    let shift_key = 4;
    let output = cipher::rotate(text, shift_key);
    let expected = "Xiwxmrk 1 2 3 xiwxmrk";
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn rotate_punctuation() {
    let text = "Let's eat, Grandma!";
    let shift_key = 21;
    let output = cipher::rotate(text, shift_key);
    let expected = "Gzo'n zvo, Bmviyhv!";
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn rotate_all_letters() {
    let text = "The quick brown fox jumps over the lazy dog.";
    let shift_key = 13;
    let output = cipher::rotate(text, shift_key);
    let expected = "Gur dhvpx oebja sbk whzcf bire gur ynml qbt.";
    assert_eq!(output, expected);
}
