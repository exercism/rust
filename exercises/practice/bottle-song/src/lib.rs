fn capitalize(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
        None => String::new(), // Handle empty string case
    }
}
fn number_to_word(n: u32) -> &'static str {
    match n {
        1 => "one",
        2 => "two",
        3 => "three",
        4 => "four",
        5 => "five",
        6 => "six",
        7 => "seven",
        8 => "eight",
        9 => "nine",
        10 => "ten",
        _ => "unknown", // Handle numbers out of range
    }
}
pub fn verse(n_bottles: u32 ) -> String {
    match n_bottles {
        1 => "One green bottle hanging on the wall,\n\
            One green bottle hanging on the wall,\n\
            And if one green bottle should accidentally fall,\n\
            There'll be no green bottles hanging on the wall.".to_string(), 
        2 => "Two green bottles hanging on the wall,\n\
            Two green bottles hanging on the wall,\n\
            And if one green bottle should accidentally fall,\n\
            There'll be one green bottle hanging on the wall.".to_string(),
        _ => format!("{} green bottles hanging on the wall,\n\
            {} green bottles hanging on the wall,\n\
            And if one green bottle should accidentally fall,\n\
            There'll be {} green bottles hanging on the wall.", capitalize(number_to_word(n_bottles)), capitalize(number_to_word(n_bottles)), number_to_word(n_bottles-1))
    }
}

pub fn recite(start_bottles: u32, take_down: u32) -> String {
    (0..take_down)
    .map(|i| verse(start_bottles-i))
    .collect::<Vec<String>>()
    .join("\n\n")
}
