extern crate bob;

#[test]
fn test_stating_something() {
    assert_eq!("Whatever.", bob::reply("Tom-ay-to, tom-aaaah-to."));
}

#[test]

fn test_shouting() {
    assert_eq!("Whoa, chill out!", bob::reply("WATCH OUT!"));
}

#[test]

fn test_shouting_gibberish() {
    assert_eq!("Whoa, chill out!", bob::reply("FCECDFCAAB"));
}

#[test]

fn test_asking() {
    assert_eq!("Sure.",
               bob::reply("Does this cryogenic chamber make me look fat?"));
}

#[test]

fn test_ask_numeric_question() {
    assert_eq!("Sure.", bob::reply("You are, what, like 15?"));
}

#[test]

fn test_asking_gibberish() {
    assert_eq!("Sure.", bob::reply("fffbbcbeab?"));
}

#[test]

fn test_exclaiming() {
    assert_eq!("Whatever.", bob::reply("Let's go make out behind the gym!"));
}

#[test]

fn test_using_acronyms_in_regular_speech() {
    assert_eq!("Whatever.",
               bob::reply("It's OK if you don't want to go to the DMV."));
}

#[test]

fn test_forceful_question() {
    assert_eq!("Calm down, I know what I'm doing!",
               bob::reply("WHAT THE HELL WERE YOU THINKING?"));
}

#[test]

fn test_shouting_numbers() {
    assert_eq!("Whoa, chill out!", bob::reply("1, 2, 3 GO!"));
}

#[test]

fn test_only_numbers() {
    assert_eq!("Whatever.", bob::reply("1, 2, 3"));
}

#[test]

fn test_question_with_only_numbers() {
    assert_eq!("Sure.", bob::reply("4?"));
}

#[test]

fn test_shouting_with_special_characters() {
    assert_eq!("Whoa, chill out!",
               bob::reply("ZOMG THE %^*@#$(*^ ZOMBIES ARE COMING!!11!!1!"));
}

#[test]

fn test_shouting_with_no_exclamation_mark() {
    assert_eq!("Whoa, chill out!", bob::reply("I HATE YOU"));
}

#[test]

fn test_statement_containing_question_mark() {
    assert_eq!("Whatever.", bob::reply("Ending with ? means a question."));
}

#[test]

fn test_non_letters_with_question() {
    assert_eq!("Sure.", bob::reply(":) ?"));
}

#[test]

fn test_prattling_on() {
    assert_eq!("Sure.", bob::reply("Wait! Hang on. Are you going to be OK?"));
}

#[test]

fn test_silence() {
    assert_eq!("Fine. Be that way!", bob::reply(""));
}

#[test]

fn test_prolonged_silence() {
    assert_eq!("Fine. Be that way!", bob::reply("          "));
}

#[test]

fn test_alternate_silence() {
    assert_eq!("Fine. Be that way!", bob::reply("\t\t\t\t\t\t\t\t\t\t"));
}

#[test]

fn test_multiple_line_question() {
    assert_eq!("Whatever.", bob::reply(
        "\nDoes this cryogenic chamber make me look fat?\nno"));
}

#[test]

fn test_starting_with_whitespace() {
    assert_eq!("Whatever.", bob::reply("         hmmmmmmm..."));
}

#[test]

fn test_ending_with_whitespace() {
    assert_eq!("Sure.",
               bob::reply("Okay if like my  spacebar  quite a bit?   "));
}

#[test]

fn test_other_whitespace() {
    assert_eq!("Fine. Be that way!", bob::reply("\n\r \t"));
}

#[test]

fn test_non_question_ending_with_whitespace() {
    assert_eq!("Whatever.",
               bob::reply("This is a statement ending with whitespace      "));
}
