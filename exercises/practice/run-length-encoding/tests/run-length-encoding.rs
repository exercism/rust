use run_length_encoding as rle;

#[test]
fn encode_empty_string() {
    let input = "";
    let output = rle::encode(input);
    let expected = "";
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn encode_single_characters_only_are_encoded_without_count() {
    let input = "XYZ";
    let output = rle::encode(input);
    let expected = "XYZ";
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn encode_string_with_no_single_characters() {
    let input = "AABBBCCCC";
    let output = rle::encode(input);
    let expected = "2A3B4C";
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn encode_single_characters_mixed_with_repeated_characters() {
    let input = "WWWWWWWWWWWWBWWWWWWWWWWWWBBBWWWWWWWWWWWWWWWWWWWWWWWWB";
    let output = rle::encode(input);
    let expected = "12WB12W3B24WB";
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn encode_multiple_whitespace_mixed_in_string() {
    let input = "  hsqq qww  ";
    let output = rle::encode(input);
    let expected = "2 hs2q q2w2 ";
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn encode_lowercase_characters() {
    let input = "aabbbcccc";
    let output = rle::encode(input);
    let expected = "2a3b4c";
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn decode_empty_string() {
    let input = "";
    let output = rle::decode(input);
    let expected = "";
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn decode_single_characters_only() {
    let input = "XYZ";
    let output = rle::decode(input);
    let expected = "XYZ";
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn decode_string_with_no_single_characters() {
    let input = "2A3B4C";
    let output = rle::decode(input);
    let expected = "AABBBCCCC";
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn decode_single_characters_with_repeated_characters() {
    let input = "12WB12W3B24WB";
    let output = rle::decode(input);
    let expected = "WWWWWWWWWWWWBWWWWWWWWWWWWBBBWWWWWWWWWWWWWWWWWWWWWWWWB";
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn decode_multiple_whitespace_mixed_in_string() {
    let input = "2 hs2q q2w2 ";
    let output = rle::decode(input);
    let expected = "  hsqq qww  ";
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn decode_lowercase_string() {
    let input = "2a3b4c";
    let output = rle::decode(input);
    let expected = "aabbbcccc";
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn consistency_encode_followed_by_decode_gives_original_string() {
    let input = "zzz ZZ  zZ";
    let output = rle::decode(&rle::encode(input));
    let expected = "zzz ZZ  zZ";
    assert_eq!(output, expected);
}
