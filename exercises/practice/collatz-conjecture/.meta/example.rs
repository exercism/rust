// return Some(x) where x is the number of steps required to reach 1
pub fn collatz(n: u64) -> Option<u64> {
    match n {
        0 => None,
        1 => Some(0),
        n if n % 2 == 0 => collatz(n / 2).map(|c| c + 1),
        n => collatz(n.checked_mul(3)?.checked_add(1)?).map(|c| c + 1),
    }
}
