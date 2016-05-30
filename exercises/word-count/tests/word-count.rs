use std::collections::HashMap;

extern crate word_count;

fn check_word_count(s: &str, pairs: Vec<(&str, u32)>) {
    // The reason for the awkward code in here is to ensure that the failure
    // message for assert_eq! is as informative as possible. A simpler
    // solution would simply check the length of the map, and then
    // check for the presence and value of each key in the given pairs vector.
    let mut m: HashMap<String, u32> = word_count::word_count(s);
    for &(k, v) in pairs.iter() {
        assert_eq!((k, m.remove(&k.to_string()).unwrap_or(0)), (k, v));
    }
    // may fail with a message that clearly shows all extra pairs in the map
    assert_eq!(m.iter().collect::<Vec<(&String,&u32)>>(), vec!());
}

#[test]
fn test_no_words() {
    check_word_count("", vec![]);
    check_word_count("-!@#$ $#() @@@", vec![]);
}

#[test]
#[ignore]
fn test_count_one_word() {
    check_word_count("word", vec![("word", 1)]);
}

#[test]
#[ignore]
fn test_count_one_of_each() {
    check_word_count(
        "one of each",
        vec![("one", 1),
             ("of", 1),
             ("each", 1)]);
}

#[test]
#[ignore]
fn test_count_multiple_occurrences() {
    check_word_count(
        "one fish two fish red fish blue fish",
        vec![("one", 1),
             ("fish", 4),
             ("two", 1),
             ("red", 1),
             ("blue", 1)]);
}

#[test]
#[ignore]
fn test_ignore_punctuation() {
    check_word_count(
        "car : carpet as java : javascript!!&@$%^&",
        vec![("car", 1),
             ("carpet", 1),
             ("as", 1),
             ("java", 1),
             ("javascript", 1)]);
}

#[test]
#[ignore]
fn test_include_numbers() {
    check_word_count(
        "testing, 1, 2 testing",
        vec![("testing", 2),
             ("1", 1),
             ("2", 1)]);
}

#[test]
#[ignore]
fn test_normalize_case() {
    check_word_count(
        "go Go GO",
        vec![("go", 3)]);
}

#[test]
#[ignore]
fn test_prefix_punctuation() {
    check_word_count(
        "!%%#testing, 1, 2 testing",
        vec![("testing", 2),
             ("1", 1),
             ("2", 1)]);
}

#[test]
#[ignore]
fn test_symbols_are_separators() {
    check_word_count(
        "hey,my_spacebar_is_broken.",
        vec![("hey", 1),
             ("my", 1),
             ("spacebar", 1),
             ("is", 1),
             ("broken", 1)]);
}
