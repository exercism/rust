use std::char;

pub fn number(s: &str) -> String {
    let digits: String = s
        .chars()
        .filter(|&c| char::is_digit_radix(c, 10))
        .collect();
    match digits.len() {
        10 => Some(digits),
        11 => match digits.as_slice().char_at(0) {
            '1' => Some(digits.as_slice().slice_from(1).into_string()),
            _   => None
        },
        _  => None
    }.unwrap_or("0000000000".to_string())
}

pub fn area_code(s: &str) -> String {
    let n = number(s);
    n.as_slice().slice_to(3).into_string()
}

pub fn pretty_print(s: &str) -> String {
    let n = number(s);
    let sl = n.as_slice();
    format!("({area}) {prefix}-{exchange}",
            area=sl.slice_to(3),
            prefix=sl.slice(3, 6),
            exchange=sl.slice_from(6))
}
