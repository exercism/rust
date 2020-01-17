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


//UPDATED
#[test]
#[ignore]
/// silence
fn test_silence() {
process_response_case({let mut hm = ::std::collections::HashMap::new(); hm.insert("heyBob", ""); hm}, "Fine. Be that way!");
}


//UPDATED
#[test]
#[ignore]
/// other whitespace
fn test_other_whitespace() {
process_response_case({let mut hm = ::std::collections::HashMap::new(); hm.insert("heyBob", "
 	"); hm}, "Fine. Be that way!");
}


//UPDATED
#[test]
#[ignore]
/// talking forcefully
fn test_talking_forcefully() {
process_response_case({let mut hm = ::std::collections::HashMap::new(); hm.insert("heyBob", "Hi there!"); hm}, "Whatever.");
}


//UPDATED
#[test]
#[ignore]
/// non-question ending with whitespace
fn test_nonquestion_ending_with_whitespace() {
process_response_case({let mut hm = ::std::collections::HashMap::new(); hm.insert("heyBob", "This is a statement ending with whitespace      "); hm}, "Whatever.");
}


//UPDATED
#[test]
#[ignore]
/// shouting numbers
fn test_shouting_numbers() {
process_response_case({let mut hm = ::std::collections::HashMap::new(); hm.insert("heyBob", "1, 2, 3 GO!"); hm}, "Whoa, chill out!");
}


//UPDATED
#[test]
#[ignore]
/// shouting with no exclamation mark
fn test_shouting_with_no_exclamation_mark() {
process_response_case({let mut hm = ::std::collections::HashMap::new(); hm.insert("heyBob", "I HATE THE DENTIST"); hm}, "Whoa, chill out!");
}


//UPDATED
#[test]
#[ignore]
/// shouting gibberish
fn test_shouting_gibberish() {
process_response_case({let mut hm = ::std::collections::HashMap::new(); hm.insert("heyBob", "FCECDFCAAB"); hm}, "Whoa, chill out!");
}


//UPDATED
#[test]
#[ignore]
/// prattling on
fn test_prattling_on() {
process_response_case({let mut hm = ::std::collections::HashMap::new(); hm.insert("heyBob", "Wait! Hang on. Are you going to be OK?"); hm}, "Sure.");
}


//UPDATED
#[test]
#[ignore]
/// starting with whitespace
fn test_starting_with_whitespace() {
process_response_case({let mut hm = ::std::collections::HashMap::new(); hm.insert("heyBob", "         hmmmmmmm..."); hm}, "Whatever.");
}


//UPDATED
#[test]
#[ignore]
/// asking gibberish
fn test_asking_gibberish() {
process_response_case({let mut hm = ::std::collections::HashMap::new(); hm.insert("heyBob", "fffbbcbeab?"); hm}, "Sure.");
}


//UPDATED
#[test]
#[ignore]
/// alternate silence
fn test_alternate_silence() {
process_response_case({let mut hm = ::std::collections::HashMap::new(); hm.insert("heyBob", "										"); hm}, "Fine. Be that way!");
}


//UPDATED
#[test]
#[ignore]
/// ending with whitespace
fn test_ending_with_whitespace() {
process_response_case({let mut hm = ::std::collections::HashMap::new(); hm.insert("heyBob", "Okay if like my  spacebar  quite a bit?   "); hm}, "Sure.");
}


//UPDATED
#[test]
#[ignore]
/// no letters
fn test_no_letters() {
process_response_case({let mut hm = ::std::collections::HashMap::new(); hm.insert("heyBob", "1, 2, 3"); hm}, "Whatever.");
}


//UPDATED
#[test]
#[ignore]
/// shouting with special characters
fn test_shouting_with_special_characters() {
process_response_case({let mut hm = ::std::collections::HashMap::new(); hm.insert("heyBob", "ZOMG THE %^*@#$(*^ ZOMBIES ARE COMING!!11!!1!"); hm}, "Whoa, chill out!");
}


//UPDATED
#[test]
#[ignore]
/// stating something
fn test_stating_something() {
process_response_case({let mut hm = ::std::collections::HashMap::new(); hm.insert("heyBob", "Tom-ay-to, tom-aaaah-to."); hm}, "Whatever.");
}


//UPDATED
#[test]
#[ignore]
/// asking a numeric question
fn test_asking_a_numeric_question() {
process_response_case({let mut hm = ::std::collections::HashMap::new(); hm.insert("heyBob", "You are, what, like 15?"); hm}, "Sure.");
}


//UPDATED
#[test]
#[ignore]
/// non-letters with question
fn test_nonletters_with_question() {
process_response_case({let mut hm = ::std::collections::HashMap::new(); hm.insert("heyBob", ":) ?"); hm}, "Sure.");
}


//UPDATED
#[test]
#[ignore]
/// question with no letters
fn test_question_with_no_letters() {
process_response_case({let mut hm = ::std::collections::HashMap::new(); hm.insert("heyBob", "4?"); hm}, "Sure.");
}


//UPDATED
#[test]
#[ignore]
/// prolonged silence
fn test_prolonged_silence() {
process_response_case({let mut hm = ::std::collections::HashMap::new(); hm.insert("heyBob", "          "); hm}, "Fine. Be that way!");
}


//UPDATED
#[test]
#[ignore]
/// multiple line question
fn test_multiple_line_question() {
process_response_case({let mut hm = ::std::collections::HashMap::new(); hm.insert("heyBob", "
Does this cryogenic chamber make me look fat?
No."); hm}, "Whatever.");
}


//UPDATED
#[test]
#[ignore]
/// using acronyms in regular speech
fn test_using_acronyms_in_regular_speech() {
process_response_case({let mut hm = ::std::collections::HashMap::new(); hm.insert("heyBob", "It's OK if you don't want to go work for NASA."); hm}, "Whatever.");
}


//UPDATED
#[test]
#[ignore]
/// statement containing question mark
fn test_statement_containing_question_mark() {
process_response_case({let mut hm = ::std::collections::HashMap::new(); hm.insert("heyBob", "Ending with ? means a question."); hm}, "Whatever.");
}


//UPDATED
#[test]
#[ignore]
/// shouting
fn test_shouting() {
process_response_case({let mut hm = ::std::collections::HashMap::new(); hm.insert("heyBob", "WATCH OUT!"); hm}, "Whoa, chill out!");
}


//UPDATED
#[test]
#[ignore]
/// asking a question
fn test_asking_a_question() {
process_response_case({let mut hm = ::std::collections::HashMap::new(); hm.insert("heyBob", "Does this cryogenic chamber make me look fat?"); hm}, "Sure.");
}


//UPDATED
#[test]
#[ignore]
/// forceful question
fn test_forceful_question() {
process_response_case({let mut hm = ::std::collections::HashMap::new(); hm.insert("heyBob", "WHAT'S GOING ON?"); hm}, "Calm down, I know what I'm doing!");
}

