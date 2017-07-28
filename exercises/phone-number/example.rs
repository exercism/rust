pub fn number(s: &str) -> Option<String> {
    let digits: String = s
        .chars()
        .filter(|&c| c.is_digit(10))
        .collect();
    match digits.len() {
        10 => Some(digits),
        11 => match digits.chars().nth(0) {
            Some('1') => Some(digits[1..].to_string()),
            _   => None
        },
        _  => None
    }
}

pub fn area_code(s: &str) -> Option<String> {
    number(s).map(|n| n[..3].to_string())
}

pub fn pretty_print(s: &str) -> String {
    number(s).map(|n|
        format!("({area}) {prefix}-{exchange}",
                area=&n[..3],
                prefix=&n[3..6],
                exchange=&n[6..])
    ).unwrap_or("invalid".to_string())
}
