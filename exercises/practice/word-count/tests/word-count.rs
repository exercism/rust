use std::collections::HashMap;

fn check_word_count(mut output: HashMap<String, u32>, pairs: &[(&str, u32)]) {
    // The reason for the awkward code in here is to ensure that the failure
    // message for assert_eq! is as informative as possible. A simpler
    // solution would simply check the length of the map, and then
    // check for the presence and value of each key in the given pairs vector.
    for &(k, v) in pairs.iter() {
        assert_eq!((k, output.remove(&k.to_string()).unwrap_or(0)), (k, v));
    }
    // may fail with a message that clearly shows all extra pairs in the map
    assert_eq!(output.iter().collect::<Vec<(&String, &u32)>>(), vec![]);
}

#[test]
fn count_one_word() {
    let input = "word";
    let output = word_count::word_count(input);
    let expected = &[("word", 1)];
    check_word_count(output, expected);
}

#[test]
#[ignore]
fn count_one_of_each_word() {
    let input = "one of each";
    let output = word_count::word_count(input);
    let expected = &[("one", 1), ("of", 1), ("each", 1)];
    check_word_count(output, expected);
}

#[test]
#[ignore]
fn multiple_occurrences_of_a_word() {
    let input = "one fish two fish red fish blue fish";
    let output = word_count::word_count(input);
    let expected = &[("one", 1), ("fish", 4), ("two", 1), ("red", 1), ("blue", 1)];
    check_word_count(output, expected);
}

#[test]
#[ignore]
fn handles_cramped_lists() {
    let input = "one,two,three";
    let output = word_count::word_count(input);
    let expected = &[("one", 1), ("two", 1), ("three", 1)];
    check_word_count(output, expected);
}

#[test]
#[ignore]
fn handles_expanded_lists() {
    let input = "one,\ntwo,\nthree";
    let output = word_count::word_count(input);
    let expected = &[("one", 1), ("two", 1), ("three", 1)];
    check_word_count(output, expected);
}

#[test]
#[ignore]
fn ignore_punctuation() {
    let input = "car: carpet as java: javascript!!&@$%^&";
    let output = word_count::word_count(input);
    let expected = &[
        ("car", 1),
        ("carpet", 1),
        ("as", 1),
        ("java", 1),
        ("javascript", 1),
    ];
    check_word_count(output, expected);
}

#[test]
#[ignore]
fn include_numbers() {
    let input = "testing, 1, 2 testing";
    let output = word_count::word_count(input);
    let expected = &[("testing", 2), ("1", 1), ("2", 1)];
    check_word_count(output, expected);
}

#[test]
#[ignore]
fn normalize_case() {
    let input = "go Go GO Stop stop";
    let output = word_count::word_count(input);
    let expected = &[("go", 3), ("stop", 2)];
    check_word_count(output, expected);
}

#[test]
#[ignore]
fn with_apostrophes() {
    let input = "'First: don't laugh. Then: don't cry. You're getting it.'";
    let output = word_count::word_count(input);
    let expected = &[
        ("first", 1),
        ("don't", 2),
        ("laugh", 1),
        ("then", 1),
        ("cry", 1),
        ("you're", 1),
        ("getting", 1),
        ("it", 1),
    ];
    check_word_count(output, expected);
}

#[test]
#[ignore]
fn with_quotations() {
    let input = "Joe can't tell between 'large' and large.";
    let output = word_count::word_count(input);
    let expected = &[
        ("joe", 1),
        ("can't", 1),
        ("tell", 1),
        ("between", 1),
        ("large", 2),
        ("and", 1),
    ];
    check_word_count(output, expected);
}

#[test]
#[ignore]
fn substrings_from_the_beginning() {
    let input = "Joe can't tell between app, apple and a.";
    let output = word_count::word_count(input);
    let expected = &[
        ("joe", 1),
        ("can't", 1),
        ("tell", 1),
        ("between", 1),
        ("app", 1),
        ("apple", 1),
        ("and", 1),
        ("a", 1),
    ];
    check_word_count(output, expected);
}

#[test]
#[ignore]
fn multiple_spaces_not_detected_as_a_word() {
    let input = " multiple   whitespaces";
    let output = word_count::word_count(input);
    let expected = &[("multiple", 1), ("whitespaces", 1)];
    check_word_count(output, expected);
}

#[test]
#[ignore]
fn alternating_word_separators_not_detected_as_a_word() {
    let input = ",\n,one,\n ,two \n 'three'";
    let output = word_count::word_count(input);
    let expected = &[("one", 1), ("two", 1), ("three", 1)];
    check_word_count(output, expected);
}

#[test]
#[ignore]
fn quotation_for_word_with_apostrophe() {
    let input = "can, can't, 'can't'";
    let output = word_count::word_count(input);
    let expected = &[("can", 1), ("can't", 2)];
    check_word_count(output, expected);
}
