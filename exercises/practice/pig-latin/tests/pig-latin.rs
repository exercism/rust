use pig_latin::*;

#[test]
fn word_beginning_with_a() {
    let input = "apple";
    let output = translate(input);
    let expected = "appleay";
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn word_beginning_with_e() {
    let input = "ear";
    let output = translate(input);
    let expected = "earay";
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn word_beginning_with_i() {
    let input = "igloo";
    let output = translate(input);
    let expected = "iglooay";
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn word_beginning_with_o() {
    let input = "object";
    let output = translate(input);
    let expected = "objectay";
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn word_beginning_with_u() {
    let input = "under";
    let output = translate(input);
    let expected = "underay";
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn word_beginning_with_a_vowel_and_followed_by_a_qu() {
    let input = "equal";
    let output = translate(input);
    let expected = "equalay";
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn word_beginning_with_p() {
    let input = "pig";
    let output = translate(input);
    let expected = "igpay";
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn word_beginning_with_k() {
    let input = "koala";
    let output = translate(input);
    let expected = "oalakay";
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn word_beginning_with_x() {
    let input = "xenon";
    let output = translate(input);
    let expected = "enonxay";
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn word_beginning_with_q_without_a_following_u() {
    let input = "qat";
    let output = translate(input);
    let expected = "atqay";
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn word_beginning_with_ch() {
    let input = "chair";
    let output = translate(input);
    let expected = "airchay";
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn word_beginning_with_qu() {
    let input = "queen";
    let output = translate(input);
    let expected = "eenquay";
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn word_beginning_with_qu_and_a_preceding_consonant() {
    let input = "square";
    let output = translate(input);
    let expected = "aresquay";
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn word_beginning_with_th() {
    let input = "therapy";
    let output = translate(input);
    let expected = "erapythay";
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn word_beginning_with_thr() {
    let input = "thrush";
    let output = translate(input);
    let expected = "ushthray";
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn word_beginning_with_sch() {
    let input = "school";
    let output = translate(input);
    let expected = "oolschay";
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn word_beginning_with_yt() {
    let input = "yttria";
    let output = translate(input);
    let expected = "yttriaay";
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn word_beginning_with_xr() {
    let input = "xray";
    let output = translate(input);
    let expected = "xrayay";
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn y_is_treated_like_a_consonant_at_the_beginning_of_a_word() {
    let input = "yellow";
    let output = translate(input);
    let expected = "ellowyay";
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn y_is_treated_like_a_vowel_at_the_end_of_a_consonant_cluster() {
    let input = "rhythm";
    let output = translate(input);
    let expected = "ythmrhay";
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn y_as_second_letter_in_two_letter_word() {
    let input = "my";
    let output = translate(input);
    let expected = "ymay";
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn a_whole_phrase() {
    let input = "quick fast run";
    let output = translate(input);
    let expected = "ickquay astfay unray";
    assert_eq!(output, expected);
}
