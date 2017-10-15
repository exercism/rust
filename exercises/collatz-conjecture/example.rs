pub fn collatz_positive(n: u64) -> u64 {
    if n == 1 {
        0
    } else  {
        1 + match n % 2 {
            0 => collatz_positive(n / 2),
            _ => collatz_positive(n * 3 + 1)
        }
    }
}

// return Ok(x) where x is the number of steps required to reach 1
pub fn collatz(n: u64) -> Result<u64, &'static str> {
    if n < 1 {
        Err("Only positive numbers are allowed")
    } else if n == 1 {
        Ok(0)
    } else {
        Ok(collatz_positive(n))
    }
}
