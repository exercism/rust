const COMMANDS:[&str; 4] = ["wink", "double blink", "close your eyes", "jump"];

pub fn commands(n: u8) -> Option<Vec<&'static str>> {
    let result: Vec<&str> = COMMANDS.iter().enumerate()
        .filter(|(i, _)| (1u8 << *i) & n != 0)
        .map(|(_, &c)| c)
        .collect();

    if result.is_empty() {
        None
    } else if n & 16 != 0 {
        Some(result.into_iter().rev().collect())
    } else {
        Some(result)
    }
}
