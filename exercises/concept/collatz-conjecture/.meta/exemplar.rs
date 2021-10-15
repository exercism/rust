pub fn collatz_next(n: u64) -> u64 {
    if n % 2 == 0 {
        n / 2
    } else {
        n * 3 + 1
    }
}

pub fn collatz_count(mut n: u64) -> u64 {
    let mut count = 0;
    while n != 1 {
        n = collatz_next(n);
        count += 1;
    }
    count
}
