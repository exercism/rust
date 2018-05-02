use std::str::FromStr;
pub fn to_decimal(s: &str) -> u32 {
    let mut acc = 0;
    for c in s.chars() {
        acc *= 3;
        match u32::from_str(&c.to_string()) {
            Ok(n) if n < 3 => acc += n,
            _ => return 0,
        }
    }
    return acc;
}
