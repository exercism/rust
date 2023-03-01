const ACTIONS: [&str; 4] = ["wink", "double blink", "close your eyes", "jump"];

pub fn actions(n: u8) -> Vec<&'static str> {
    let result: Vec<&str> = ACTIONS
        .iter()
        .enumerate()
        .filter(|(i, _)| (1u8 << *i) & n != 0)
        .map(|(_, &c)| c)
        .collect();

    if n & 16 != 0 {
        result.into_iter().rev().collect()
    } else {
        result
    }
}
   