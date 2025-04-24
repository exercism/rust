pub fn square_of_sum(n: u32) -> u32 {
    sum(n).pow(2)
}

pub fn sum_of_squares(n: u32) -> u32 {
    (1..=n)
    .map(|m| m*m)
    .sum()
}

pub fn difference(n: u32) -> u32 {
    square_of_sum(n) - sum_of_squares(n)
}

fn sum(n: u32) -> u32 {
    n * (n+1) / 2
}
