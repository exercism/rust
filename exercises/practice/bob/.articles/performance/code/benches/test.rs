use bob_reply::{
    reply_array, reply_if_chain, reply_match, reply_state_machine,
    reply_state_machine_ascii_optimized,
};

const BASIC_LOREM_IPSUM_100: &str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. In arcu justo, facilisis vel leo et sapien";
const BASIC_LOREM_IPSUM_1000: &str = include_str!("./1000_lorem_ipsum.txt");
const BASIC_LOREM_IPSUM_10000: &str = include_str!("./10000_lorem_ipsum.txt");

const BASIC_LOREM_IPSUM_NON_ASCII_100: &str = "Lorem ipsum dolor sit amet, consectetur adipiscing èlit. În arcu justo, facilisis vel leo et sapien";
const BASIC_LOREM_IPSUM_NON_ASCII_1000: &str = include_str!("./1000_lorem_ipsum_non_ascii.txt");
const BASIC_LOREM_IPSUM_NON_ASCII_10000: &str = include_str!("./10000_lorem_ipsum_non_ascii.txt");

#[test]
fn test_normal_question() {
    let inputs = vec![
        String::from("?"),
        String::from("a?"),
        String::from("Hello there?"), // ~10
        format!("{BASIC_LOREM_IPSUM_100}?"),
        format!("{BASIC_LOREM_IPSUM_1000}?"),
        format!("{BASIC_LOREM_IPSUM_10000}?"),
        format!("{BASIC_LOREM_IPSUM_NON_ASCII_100}?"),
        format!("{BASIC_LOREM_IPSUM_NON_ASCII_1000}?"),
        format!("{BASIC_LOREM_IPSUM_NON_ASCII_10000}?"),
    ];
    let expected = "Sure.";
    for input in inputs {
        assert_eq!(reply_array(input.as_str()), expected);
        assert_eq!(reply_if_chain(input.as_str()), expected);
        assert_eq!(reply_match(input.as_str()), expected);
        assert_eq!(reply_state_machine(input.as_str()), expected);
        assert_eq!(
            reply_state_machine_ascii_optimized(input.as_str()),
            expected
        );
    }
}

#[test]
fn test_yell() {
    let inputs = vec![
        String::from("A"),
        String::from("HELLO THERE"), // ~10
        format!("{}", BASIC_LOREM_IPSUM_100.to_uppercase()),
        format!("{}", BASIC_LOREM_IPSUM_1000.to_uppercase()),
        format!("{}", BASIC_LOREM_IPSUM_10000.to_uppercase()),
        format!("{}", BASIC_LOREM_IPSUM_NON_ASCII_100.to_uppercase()),
        format!("{}", BASIC_LOREM_IPSUM_NON_ASCII_1000.to_uppercase()),
        format!("{}", BASIC_LOREM_IPSUM_NON_ASCII_10000.to_uppercase()),
    ];
    let expected = "Whoa, chill out!";
    for input in inputs {
        assert_eq!(reply_array(input.as_str()), expected);
        assert_eq!(reply_if_chain(input.as_str()), expected);
        assert_eq!(reply_match(input.as_str()), expected);
        assert_eq!(reply_state_machine(input.as_str()), expected);
        assert_eq!(
            reply_state_machine_ascii_optimized(input.as_str()),
            expected
        );
    }
}

#[test]
fn test_yell_question() {
    let inputs = vec![
        String::from("A?"),
        String::from("HELLO THERE?"), // ~10
        format!("{}?", BASIC_LOREM_IPSUM_100.to_uppercase()),
        format!("{}?", BASIC_LOREM_IPSUM_1000.to_uppercase()),
        format!("{}?", BASIC_LOREM_IPSUM_10000.to_uppercase()),
        format!("{}?", BASIC_LOREM_IPSUM_NON_ASCII_100.to_uppercase()),
        format!("{}?", BASIC_LOREM_IPSUM_NON_ASCII_1000.to_uppercase()),
        format!("{}?", BASIC_LOREM_IPSUM_NON_ASCII_10000.to_uppercase()),
    ];
    let expected = "Calm down, I know what I'm doing!";
    for input in inputs {
        assert_eq!(reply_array(input.as_str()), expected);
        assert_eq!(reply_if_chain(input.as_str()), expected);
        assert_eq!(reply_match(input.as_str()), expected);
        assert_eq!(reply_state_machine(input.as_str()), expected);
        assert_eq!(
            reply_state_machine_ascii_optimized(input.as_str()),
            expected
        );
    }
}

#[test]
fn test_whatever() {
    let inputs = vec![
        String::from("a"),
        String::from("Hello there"), // ~10
        format!("{BASIC_LOREM_IPSUM_100}"),
        format!("{BASIC_LOREM_IPSUM_1000}"),
        format!("{BASIC_LOREM_IPSUM_10000}"),
        format!("{BASIC_LOREM_IPSUM_NON_ASCII_100}"),
        format!("{BASIC_LOREM_IPSUM_NON_ASCII_1000}"),
        format!("{BASIC_LOREM_IPSUM_NON_ASCII_10000}"),
    ];
    let expected = "Whatever.";
    for input in inputs {
        assert_eq!(reply_array(input.as_str()), expected);
        assert_eq!(reply_if_chain(input.as_str()), expected);
        assert_eq!(reply_match(input.as_str()), expected);
        assert_eq!(reply_state_machine(input.as_str()), expected);
        assert_eq!(
            reply_state_machine_ascii_optimized(input.as_str()),
            expected
        );
    }
}
