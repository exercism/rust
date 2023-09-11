fn process_response_case(phrase: &str, expected_response: &str) {
    assert_eq!(bob::reply(phrase), expected_response);
}

#[test]
/// stating something
fn stating_something() {
    process_response_case("Tom-ay-to, tom-aaaah-to.", "Whatever.");
}

#[test]
#[ignore]
/// ending with whitespace
fn ending_with_whitespace() {
    process_response_case("Okay if like my  spacebar  quite a bit?   ", "Sure.");
}

#[test]
#[ignore]
/// shouting numbers
fn shouting_numbers() {
    process_response_case("1, 2, 3 GO!", "Whoa, chill out!");
}

#[test]
#[ignore]
/// other whitespace
fn other_whitespace() {
    process_response_case("\r\r 	", "Fine. Be that way!");
}

#[test]
#[ignore]
/// shouting with special characters
fn shouting_with_special_characters() {
    process_response_case(
        "ZOMG THE %^*@#$(*^ ZOMBIES ARE COMING!!11!!1!",
        "Whoa, chill out!",
    );
}

#[test]
#[ignore]
/// talking forcefully
fn talking_forcefully() {
    process_response_case("Hi there!", "Whatever.");
}

#[test]
#[ignore]
/// prattling on
fn prattling_on() {
    process_response_case("Wait! Hang on. Are you going to be OK?", "Sure.");
}

#[test]
#[ignore]
/// forceful question
fn forceful_question() {
    process_response_case("WHAT'S GOING ON?", "Calm down, I know what I'm doing!");
}

#[test]
#[ignore]
/// shouting with no exclamation mark
fn shouting_with_no_exclamation_mark() {
    process_response_case("I HATE THE DENTIST", "Whoa, chill out!");
}

#[test]
#[ignore]
/// asking gibberish
fn asking_gibberish() {
    process_response_case("fffbbcbeab?", "Sure.");
}

#[test]
#[ignore]
/// question with no letters
fn question_with_no_letters() {
    process_response_case("4?", "Sure.");
}

#[test]
#[ignore]
/// no letters
fn no_letters() {
    process_response_case("1, 2, 3", "Whatever.");
}

#[test]
#[ignore]
/// statement containing question mark
fn statement_containing_question_mark() {
    process_response_case("Ending with ? means a question.", "Whatever.");
}

//NEW
#[test]
#[ignore]
/// multiple line question
fn multiple_line_question() {
    process_response_case(
        "\rDoes this cryogenic chamber make me look fat?\rNo.",
        "Whatever.",
    );
}

#[test]
#[ignore]
/// non-question ending with whitespace
fn nonquestion_ending_with_whitespace() {
    process_response_case(
        "This is a statement ending with whitespace      ",
        "Whatever.",
    );
}

#[test]
#[ignore]
/// shouting
fn shouting() {
    process_response_case("WATCH OUT!", "Whoa, chill out!");
}

#[test]
#[ignore]
/// non-letters with question
fn nonletters_with_question() {
    process_response_case(":) ?", "Sure.");
}

#[test]
#[ignore]
/// shouting gibberish
fn shouting_gibberish() {
    process_response_case("FCECDFCAAB", "Whoa, chill out!");
}

#[test]
#[ignore]
/// asking a question
fn asking_a_question() {
    process_response_case("Does this cryogenic chamber make me look fat?", "Sure.");
}

#[test]
#[ignore]
/// asking a numeric question
fn asking_a_numeric_question() {
    process_response_case("You are, what, like 15?", "Sure.");
}

#[test]
#[ignore]
/// silence
fn silence() {
    process_response_case("", "Fine. Be that way!");
}

#[test]
#[ignore]
/// starting with whitespace
fn starting_with_whitespace() {
    process_response_case("         hmmmmmmm...", "Whatever.");
}

#[test]
#[ignore]
/// using acronyms in regular speech
fn using_acronyms_in_regular_speech() {
    process_response_case(
        "It's OK if you don't want to go work for NASA.",
        "Whatever.",
    );
}

#[test]
#[ignore]
/// alternate silence
fn alternate_silence() {
    process_response_case("										", "Fine. Be that way!");
}

#[test]
#[ignore]
/// prolonged silence
fn prolonged_silence() {
    process_response_case("          ", "Fine. Be that way!");
}
