pub fn number(s: &str) -> String {
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
    }.unwrap_or("0000000000".to_string())
}

pub fn area_code(s: &str) -> String {
    let n = number(s);
    n[..3].to_string()
}

pub fn pretty_print(s: &str) -> String {
    let n = number(s);
    format!("({area}) {prefix}-{exchange}",
            area=&n[..3],
            prefix=&n[3..6],
            exchange=&n[6..])
}
