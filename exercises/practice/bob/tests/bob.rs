use bob::*;

#[test]
fn stating_something() {
    assert_eq!(reply("Tom-ay-to, tom-aaaah-to."), "Whatever.");
}

#[test]
#[ignore]
fn shouting() {
    assert_eq!(reply("WATCH OUT!"), "Whoa, chill out!");
}

#[test]
#[ignore]
fn shouting_gibberish() {
    assert_eq!(reply("FCECDFCAAB"), "Whoa, chill out!");
}

#[test]
#[ignore]
fn asking_a_question() {
    assert_eq!(
        reply("Does this cryogenic chamber make me look fat?"),
        "Sure."
    );
}

#[test]
#[ignore]
fn asking_a_numeric_question() {
    assert_eq!(reply("You are, what, like 15?"), "Sure.");
}

#[test]
#[ignore]
fn asking_gibberish() {
    assert_eq!(reply("fffbbcbeab?"), "Sure.");
}

#[test]
#[ignore]
fn talking_forcefully() {
    assert_eq!(reply("Hi there!"), "Whatever.");
}

#[test]
#[ignore]
fn using_acronyms_in_regular_speech() {
    assert_eq!(
        reply("It's OK if you don't want to go work for NASA."),
        "Whatever."
    );
}

#[test]
#[ignore]
fn forceful_question() {
    assert_eq!(
        reply("WHAT'S GOING ON?"),
        "Calm down, I know what I'm doing!"
    );
}

#[test]
#[ignore]
fn shouting_numbers() {
    assert_eq!(reply("1, 2, 3 GO!"), "Whoa, chill out!");
}

#[test]
#[ignore]
fn no_letters() {
    assert_eq!(reply("1, 2, 3"), "Whatever.");
}

#[test]
#[ignore]
fn question_with_no_letters() {
    assert_eq!(reply("4?"), "Sure.");
}

#[test]
#[ignore]
fn shouting_with_special_characters() {
    assert_eq!(
        reply("ZOMG THE %^*@#$(*^ ZOMBIES ARE COMING!!11!!1!"),
        "Whoa, chill out!"
    );
}

#[test]
#[ignore]
fn shouting_with_no_exclamation_mark() {
    assert_eq!(reply("I HATE THE DENTIST"), "Whoa, chill out!");
}

#[test]
#[ignore]
fn statement_containing_question_mark() {
    assert_eq!(reply("Ending with ? means a question."), "Whatever.");
}

#[test]
#[ignore]
fn non_letters_with_question() {
    assert_eq!(reply(":) ?"), "Sure.");
}

#[test]
#[ignore]
fn prattling_on() {
    assert_eq!(reply("Wait! Hang on. Are you going to be OK?"), "Sure.");
}

#[test]
#[ignore]
fn silence() {
    assert_eq!(reply(""), "Fine. Be that way!");
}

#[test]
#[ignore]
fn prolonged_silence() {
    assert_eq!(reply("          "), "Fine. Be that way!");
}

#[test]
#[ignore]
fn alternate_silence() {
    assert_eq!(reply("\t\t\t\t\t\t\t\t\t\t"), "Fine. Be that way!");
}

#[test]
#[ignore]
fn starting_with_whitespace() {
    assert_eq!(reply("         hmmmmmmm..."), "Whatever.");
}

#[test]
#[ignore]
fn ending_with_whitespace() {
    assert_eq!(reply("Okay if like my  spacebar  quite a bit?   "), "Sure.");
}

#[test]
#[ignore]
fn other_whitespace() {
    assert_eq!(reply("\n\r \t"), "Fine. Be that way!");
}

#[test]
#[ignore]
fn non_question_ending_with_whitespace() {
    assert_eq!(
        reply("This is a statement ending with whitespace      "),
        "Whatever."
    );
}

#[test]
#[ignore]
fn multiple_line_question() {
    assert_eq!(
        reply("\nDoes this cryogenic chamber make\n me look fat?"),
        "Sure."
    );
}
