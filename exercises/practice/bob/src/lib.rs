pub fn reply(message: &str) -> &str {

    let trimed = message.trim();

    if trimed.is_empty() {
        return "Fine. Be that way!"
    }

    let is_question = trimed.ends_with("?");
    let is_yelled_vec: Vec<char> = trimed.chars().filter(|c| c.is_alphabetic()).collect();
    let is_yelled = is_yelled_vec.iter().all(|c| c.is_uppercase()) && !is_yelled_vec.is_empty();

    match (is_question, is_yelled) {
        (true, true) => "Calm down, I know what I'm doing!",
        (true, false) => "Sure.",
        (false, true) => "Whoa, chill out!",
        (false, false) => "Whatever."
    }
}
