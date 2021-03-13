pub fn reply(message: &str) -> &str {
    if is_silence(message) {
        "Fine. Be that way!"
    } else if is_yelling(message) && is_question(message) {
        "Calm down, I know what I'm doing!"
    } else if is_yelling(message) {
        "Whoa, chill out!"
    } else if is_question(message) {
        "Sure."
    } else {
        "Whatever."
    }
}

fn is_silence(message: &str) -> bool {
    message.trim().is_empty()
}

fn is_yelling(message: &str) -> bool {
    let s = message.trim_matches(|c: char| !c.is_alphabetic());
    !s.is_empty() && s.to_uppercase() == s
}

fn is_question(message: &str) -> bool {
    message.trim_end().ends_with('?')
}
