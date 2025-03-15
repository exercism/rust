// Reply using match
pub fn reply_match(msg: &str) -> &str {
    let message = msg.trim_end();
    if message.is_empty() {
        return "Fine. Be that way!";
    }

    let is_questioning = message.ends_with('?');
    let is_yelling =
        message.chars().any(|ch| ch.is_alphabetic()) && message == message.to_uppercase();

    match (is_yelling, is_questioning) {
        (true, true) => "Calm down, I know what I'm doing!",
        (true, _) => "Whoa, chill out!",
        (_, true) => "Sure.",
        _ => "Whatever.",
    }
}

// Reply using if chain
pub fn reply_if_chain(msg: &str) -> &str {
    let message = msg.trim_end();
    if message.is_empty() {
        return "Fine. Be that way!";
    }

    let is_questioning = message.ends_with('?');
    let is_yelling =
        message.chars().any(|ch| ch.is_alphabetic()) && message == message.to_uppercase();

    if is_yelling {
        return if is_questioning {
            "Calm down, I know what I'm doing!"
        } else {
            "Whoa, chill out!"
        };
    }
    if is_questioning {
        return "Sure.";
    }
    "Whatever."
}

// Reply using array
const ANSWERS: &[&str] = &[
    "Whatever.",
    "Sure.",
    "Whoa, chill out!",
    "Calm down, I know what I'm doing!",
];

pub fn reply_array(msg: &str) -> &str {
    let message = msg.trim_end();
    if message.is_empty() {
        return "Fine. Be that way!";
    }

    let is_questioning = if message.ends_with('?') { 1 } else { 0 };
    let is_yelling =
        if message.chars().any(|ch| ch.is_alphabetic()) && message == message.to_uppercase() {
            2
        } else {
            0
        };

    ANSWERS[is_questioning + is_yelling]
}

// Reply using state machine
#[derive(Hash, Eq, PartialEq, Debug)]
enum State {
    Initial,
    HasQuestionMark,
    NoQuestionMarkUpperCase,
    NoQuestionMarkUndefined,
    QuestionMarkUpperCase,
    QuestionMarkUndefined,
}
enum NextStateResult {
    NextState(State),
    FinalResult(&'static str),
}
impl State {
    fn next_state(self, c: char) -> NextStateResult {
        match self {
            State::Initial => {
                if c == '?' {
                    return NextStateResult::NextState(State::HasQuestionMark);
                }

                if c.is_lowercase() {
                    return NextStateResult::FinalResult(WHATEVER);
                }
                if c.is_uppercase() {
                    return NextStateResult::NextState(State::NoQuestionMarkUpperCase);
                }
                NextStateResult::NextState(State::NoQuestionMarkUndefined)
            }
            State::HasQuestionMark => {
                if c.is_uppercase() {
                    return NextStateResult::NextState(State::QuestionMarkUpperCase);
                }
                if c.is_lowercase() {
                    return NextStateResult::FinalResult(SURE);
                }
                NextStateResult::NextState(State::QuestionMarkUndefined)
            }
            // optimize
            State::NoQuestionMarkUpperCase => {
                if c.is_lowercase() {
                    return NextStateResult::FinalResult(WHATEVER);
                }
                NextStateResult::NextState(self)
            }
            State::NoQuestionMarkUndefined => {
                if c.is_lowercase() {
                    return NextStateResult::FinalResult(WHATEVER);
                }
                if c.is_uppercase() {
                    return NextStateResult::NextState(State::NoQuestionMarkUpperCase);
                }
                NextStateResult::NextState(self)
            }
            State::QuestionMarkUpperCase => {
                if c.is_lowercase() {
                    return NextStateResult::FinalResult(SURE);
                }
                NextStateResult::NextState(self)
            }
            State::QuestionMarkUndefined => {
                if c.is_lowercase() {
                    return NextStateResult::FinalResult(SURE);
                }
                if c.is_uppercase() {
                    return NextStateResult::NextState(State::QuestionMarkUpperCase);
                }
                NextStateResult::NextState(self)
            }
        }
    }
}

const FINE_BE_THAT_WAY: &str = "Fine. Be that way!";
const WHATEVER: &str = "Whatever.";
const SURE: &str = "Sure.";
const CHILL_OUT: &str = "Whoa, chill out!";
const CALM_DOWN: &str = "Calm down, I know what I'm doing!";

pub fn reply_state_machine(message: &str) -> &str {
    let message = message.trim_end();
    if message.is_empty() {
        return FINE_BE_THAT_WAY;
    }
    let mut state = State::Initial;
    for c in message.chars().rev() {
        match state.next_state(c) {
            NextStateResult::NextState(s) => state = s,
            NextStateResult::FinalResult(r) => return r,
        }
    }
    match state {
        State::HasQuestionMark | State::QuestionMarkUndefined => SURE,
        State::NoQuestionMarkUpperCase => CHILL_OUT,
        State::NoQuestionMarkUndefined => WHATEVER,
        State::QuestionMarkUpperCase => CALM_DOWN,
        _ => panic!("Unexpected final state"),
    }
}

pub fn reply_state_machine_ascii_optimized(message: &str) -> &str {
    let message = message.trim_end();
    if message.is_empty() {
        return FINE_BE_THAT_WAY;
    }
    let mut state = State::Initial;
    // May be even more optimized:
    // Initiate supposing the input is ascii: while c.is_ascii()...
    // When the first non-ascii is found, change the approach
    if message.is_ascii() {
        for &c in message.as_bytes().iter().rev() {
            match state.next_state(char::from(c)) {
                NextStateResult::NextState(s) => state = s,
                NextStateResult::FinalResult(r) => return r,
            }
        }
    } else {
        for c in message.chars().rev() {
            match state.next_state(c) {
                NextStateResult::NextState(s) => state = s,
                NextStateResult::FinalResult(r) => return r,
            }
        }
    }
    match state {
        State::HasQuestionMark | State::QuestionMarkUndefined => SURE,
        State::NoQuestionMarkUpperCase => CHILL_OUT,
        State::NoQuestionMarkUndefined => WHATEVER,
        State::QuestionMarkUpperCase => CALM_DOWN,
        _ => panic!("Unexpected final state"),
    }
}

macro_rules! test_reply {
    ( $( ($function_name:ident, $module_name:ident)),* ) => {

            $(
                mod $module_name{
                    #[allow(unused_imports)]
                    use super::*;

                    #[test]
                    fn stating_something() {
                        assert_eq!($function_name("Tom-ay-to, tom-aaaah-to."), "Whatever.");
                    }

                    #[test]
                    fn shouting() {
                        assert_eq!($function_name("WATCH OUT!"), "Whoa, chill out!");
                    }

                    #[test]
                    fn shouting_gibberish() {
                        assert_eq!($function_name("FCECDFCAAB"), "Whoa, chill out!");
                    }

                    #[test]
                    fn asking_a_question() {
                        assert_eq!(
                            $function_name("Does this cryogenic chamber make me look fat?"),
                            "Sure."
                        );
                    }

                    #[test]
                    fn asking_a_numeric_question() {
                        assert_eq!($function_name("You are, what, like 15?"), "Sure.");
                    }

                    #[test]
                    fn asking_gibberish() {
                        assert_eq!($function_name("fffbbcbeab?"), "Sure.");
                    }

                    #[test]
                    fn talking_forcefully() {
                        assert_eq!($function_name("Hi there!"), "Whatever.");
                    }

                    #[test]
                    fn using_acronyms_in_regular_speech() {
                        assert_eq!(
                            $function_name("It's OK if you don't want to go work for NASA."),
                            "Whatever."
                        );
                    }

                    #[test]
                    fn forceful_question() {
                        assert_eq!(
                            $function_name("WHAT'S GOING ON?"),
                            "Calm down, I know what I'm doing!"
                        );
                    }

                    #[test]
                    fn shouting_numbers() {
                        assert_eq!($function_name("1, 2, 3 GO!"), "Whoa, chill out!");
                    }

                    #[test]
                    fn no_letters() {
                        assert_eq!($function_name("1, 2, 3"), "Whatever.");
                    }

                    #[test]
                    fn question_with_no_letters() {
                        assert_eq!($function_name("4?"), "Sure.");
                    }

                    #[test]
                    fn shouting_with_special_characters() {
                        assert_eq!(
                            $function_name("ZOMG THE %^*@#$(*^ ZOMBIES ARE COMING!!11!!1!"),
                            "Whoa, chill out!"
                        );
                    }

                    #[test]
                    fn shouting_with_no_exclamation_mark() {
                        assert_eq!($function_name("I HATE THE DENTIST"), "Whoa, chill out!");
                    }

                    #[test]
                    fn statement_containing_question_mark() {
                        assert_eq!($function_name("Ending with ? means a question."), "Whatever.");
                    }

                    #[test]
                    fn non_letters_with_question() {
                        assert_eq!($function_name(":) ?"), "Sure.");
                    }

                    #[test]
                    fn prattling_on() {
                        assert_eq!($function_name("Wait! Hang on. Are you going to be OK?"), "Sure.");
                    }

                    #[test]
                    fn silence() {
                        assert_eq!($function_name(""), "Fine. Be that way!");
                    }

                    #[test]
                    fn prolonged_silence() {
                        assert_eq!($function_name("          "), "Fine. Be that way!");
                    }

                    #[test]
                    fn alternate_silence() {
                        assert_eq!($function_name("\t\t\t\t\t\t\t\t\t\t"), "Fine. Be that way!");
                    }

                    #[test]
                    fn starting_with_whitespace() {
                        assert_eq!($function_name("         hmmmmmmm..."), "Whatever.");
                    }

                    #[test]
                    fn ending_with_whitespace() {
                        assert_eq!($function_name("Okay if like my  spacebar  quite a bit?   "), "Sure.");
                    }

                    #[test]
                    fn other_whitespace() {
                        assert_eq!($function_name("\n\r \t"), "Fine. Be that way!");
                    }

                    #[test]
                    fn non_question_ending_with_whitespace() {
                        assert_eq!(
                            $function_name("This is a statement ending with whitespace      "),
                            "Whatever."
                        );
                    }

                    #[test]
                    fn multiple_line_question() {
                        assert_eq!(
                            $function_name("\nDoes this cryogenic chamber make\n me look fat?"),
                            "Sure."
                        );
                    }
                }
            )*
    };
}

mod tests {
    #[allow(unused_imports)]
    use super::*;

    test_reply!(
        (reply_match, reply_match_test),
        (reply_if_chain, reply_if_chain_test),
        (reply_array, reply_array_test),
        (reply_state_machine, reply_state_machine_test),
        (
            reply_state_machine_ascii_optimized,
            reply_state_machine_ascii_optimized_test
        )
    );
}
