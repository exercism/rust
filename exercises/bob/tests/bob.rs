use bob;

fn process_response_case(phrase: &str, expected_response: &str) {
    assert_eq!(bob::reply(phrase), expected_response);
}

#[test]
fn test_stating_something() {
    process_response_case("Tom-ay-to, tom-aaaah-to.", "Whatever.");
}

#[test]
#[ignore]
fn test_shouting() {
    process_response_case("WATCH OUT!", "Whoa, chill out!");
}

#[test]
#[ignore]
fn test_shouting_gibberish() {
    process_response_case("FCECDFCAAB", "Whoa, chill out!");
}

#[test]
#[ignore]
fn test_asking_a_question() {
    process_response_case("Does this cryogenic chamber make me look fat?", "Sure.");
}

#[test]
#[ignore]
fn test_asking_a_numeric_question() {
    process_response_case("You are, what, like 15?", "Sure.");
}

#[test]
#[ignore]
fn test_asking_gibberish() {
    process_response_case("fffbbcbeab?", "Sure.");
}

#[test]
#[ignore]
fn test_talking_forcefully() {
    process_response_case("Let's go make out behind the gym!", "Whatever.");
}

#[test]
#[ignore]
fn test_using_acronyms_in_regular_speech() {
    process_response_case("It's OK if you don't want to go to the DMV.", "Whatever.");
}

#[test]
#[ignore]
fn test_forceful_question() {
    process_response_case(
        "WHAT THE HELL WERE YOU THINKING?",
        "Calm down, I know what I'm doing!",
    );
}

#[test]
#[ignore]
fn test_shouting_numbers() {
    process_response_case("1, 2, 3 GO!", "Whoa, chill out!");
}

#[test]
#[ignore]
fn test_no_letters() {
    process_response_case("1, 2, 3", "Whatever.");
}

#[test]
#[ignore]
fn test_question_with_no_letters() {
    process_response_case("4?", "Sure.");
}

#[test]
#[ignore]
fn test_shouting_with_special_characters() {
    process_response_case(
        "ZOMG THE %^*@#$(*^ ZOMBIES ARE COMING!!11!!1!",
        "Whoa, chill out!",
    );
}

#[test]
#[ignore]
fn test_shouting_with_no_exclamation_mark() {
    process_response_case("I HATE THE DMV", "Whoa, chill out!");
}

#[test]
#[ignore]
fn test_statement_containing_question_mark() {
    process_response_case("Ending with ? means a question.", "Whatever.");
}

#[test]
#[ignore]
fn test_nonletters_with_question() {
    process_response_case(":) ?", "Sure.");
}

#[test]
#[ignore]
fn test_prattling_on() {
    process_response_case("Wait! Hang on. Are you going to be OK?", "Sure.");
}

#[test]
#[ignore]
fn test_silence() {
    process_response_case("", "Fine. Be that way!");
}

#[test]
#[ignore]
fn test_prolonged_silence() {
    process_response_case("          ", "Fine. Be that way!");
}

#[test]
#[ignore]
fn test_alternate_silence() {
    process_response_case("\t\t\t\t\t\t\t\t\t\t", "Fine. Be that way!");
}

#[test]
#[ignore]
fn test_multiple_line_question() {
    process_response_case(
        "\nDoes this cryogenic chamber make me look fat?\nNo.",
        "Whatever.",
    );
}

#[test]
#[ignore]
fn test_starting_with_whitespace() {
    process_response_case("         hmmmmmmm...", "Whatever.");
}

#[test]
#[ignore]
fn test_ending_with_whitespace() {
    process_response_case("Okay if like my  spacebar  quite a bit?   ", "Sure.");
}

#[test]
#[ignore]
fn test_other_whitespace() {
    process_response_case("\n\r \t", "Fine. Be that way!");
}

#[test]
#[ignore]
fn test_nonquestion_ending_with_whitespace() {
    process_response_case(
        "This is a statement ending with whitespace      ",
        "Whatever.",
    );
}
