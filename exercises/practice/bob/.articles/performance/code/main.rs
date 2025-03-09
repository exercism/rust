#![feature(test)]
extern crate test;
use test::Bencher;

fn main() {
    println!("Hello, world!");
}

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
const ANSWERS: &'static [&'static str] = &[
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
enum State {
    Initial,
    HasQuestionMark,
    NoQuestionMarkUpperCase,
    NoQuestionMarkUndefined,
    QuestionMarkUpperCase,
    QuestionMarkUndefined,
}

const FINE_BE_THAT_WAY: &str = "Fine. Be that way!";
const WHATEVER: &str = "Whatever.";
const SURE: &str = "Sure.";
const CHILL_OUT: &str = "Whoa, chill out!";
const CALM_DOWN: &str = "Calm down, I know what I'm doing!";

pub fn reply_state_machine(message: &str) -> &str {
    let message = message.trim();
    if message.is_empty() {
        return FINE_BE_THAT_WAY;
    }
    let mut state = State::Initial;
    for c in message.chars().rev() {
        match state {
            State::Initial => {
                state = if c == '?' {
                    State::HasQuestionMark
                } else {
                    if c.is_lowercase() {
                        return WHATEVER;
                    }
                    if c.is_uppercase() {
                        State::NoQuestionMarkUpperCase
                    } else {
                        State::NoQuestionMarkUndefined
                    }
                };
            }
            State::HasQuestionMark => {
                state = if c.is_uppercase() {
                    State::QuestionMarkUpperCase
                } else {
                    State::QuestionMarkUndefined
                };
            }
            State::NoQuestionMarkUpperCase => {
                if c.is_lowercase() {
                    return WHATEVER;
                }
            }
            State::NoQuestionMarkUndefined => {
                if c.is_lowercase() {
                    return WHATEVER;
                }
                if c.is_uppercase() {
                    state = State::NoQuestionMarkUpperCase;
                }
            }
            State::QuestionMarkUpperCase => {
                if c.is_lowercase() {
                    return SURE;
                }
            }
            State::QuestionMarkUndefined => {
                if c.is_lowercase() {
                    return SURE;
                }
                if c.is_uppercase() {
                    state = State::QuestionMarkUpperCase;
                }
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

#[bench]
/// multiple line question for match
fn multiple_line_question_match(b: &mut Bencher) {
    b.iter(|| {
        reply_match("\rDoes this cryogenic chamber make me look fat?\rNo.");
        reply_match("  ");
        reply_match("Does this cryogenic chamber make me look fat?");
        reply_match("WHAT'S GOING ON?");
        reply_match("ZOMG THE %^*@#$(*^ ZOMBIES ARE COMING!!11!!1!");
    });
}

#[bench]
/// multiple line question for if statements
fn multiple_line_question_if(b: &mut Bencher) {
    b.iter(|| {
        reply_if_chain("\rDoes this cryogenic chamber make me look fat?\rNo.");
        reply_if_chain("  ");
        reply_if_chain("Does this cryogenic chamber make me look fat?");
        reply_if_chain("WHAT'S GOING ON?");
        reply_if_chain("ZOMG THE %^*@#$(*^ ZOMBIES ARE COMING!!11!!1!");
    });
}

#[bench]
/// multiple line question for answer array
fn multiple_line_question_array(b: &mut Bencher) {
    b.iter(|| {
        reply_array("\rDoes this cryogenic chamber make me look fat?\rNo.");
        reply_array("  ");
        reply_array("Does this cryogenic chamber make me look fat?");
        reply_array("WHAT'S GOING ON?");
        reply_array("ZOMG THE %^*@#$(*^ ZOMBIES ARE COMING!!11!!1!");
    });
}

#[bench]
/// multiple line question for state machine
fn multiple_line_question_state_machine(b: &mut Bencher) {
    b.iter(|| {
        reply_state_machine("\rDoes this cryogenic chamber make me look fat?\rNo.");
        reply_state_machine("  ");
        reply_state_machine("Does this cryogenic chamber make me look fat?");
        reply_state_machine("WHAT'S GOING ON?");
        reply_state_machine("ZOMG THE %^*@#$(*^ ZOMBIES ARE COMING!!11!!1!");
    });
}
