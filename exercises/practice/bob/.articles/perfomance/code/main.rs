#![feature(test)]
extern crate test;
use test::Bencher;

fn main() {
    println!("Hello, world!");
}

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

#[bench]
/// multiple line question for match
fn test_multiple_line_question_match(b: &mut Bencher) {
    b.iter(|| reply_match("\rDoes this cryogenic chamber make me look fat?\rNo."));
}

#[bench]
/// multiple line question for if statements
fn test_multiple_line_question_if(b: &mut Bencher) {
    b.iter(|| reply_if_chain("\rDoes this cryogenic chamber make me look fat?\rNo."));
}

#[bench]
/// multiple line question for answer array
fn test_multiple_line_question_array(b: &mut Bencher) {
    b.iter(|| reply_array("\rDoes this cryogenic chamber make me look fat?\rNo."));
}
