pub fn number(s: &str) -> Option<String> {
    let digits: String = s.chars().filter(|&c| c.is_digit(10)).collect();
    match digits.len() {
        10 => {
            match (digits.chars().nth(0), digits.chars().nth(3)) {
                (Some('0'), _) => None,
                (Some('1'), _) => None,
                (_, Some('0')) => None,
                (_, Some('1')) => None,
                _ => Some(digits),
            }
        }
        11 => {
            match digits.chars().nth(0) {
                Some('1') => Some(digits[1..].to_string()),
                _ => None,
            }
        }
        _ => None,
    }
}
