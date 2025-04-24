pub fn square(s: u32) -> u64 {
    2_u64.pow(s-1)
}

pub fn total() -> u64 {
    (1..=64).map(square).sum::<u64>()
}
