pub fn square_of_sum(n: usize) -> usize {
    let sum = n * (n + 1) / 2;
    sum * sum
}

pub fn sum_of_squares(n: usize) -> usize {
    (0..n+1).map(|x| x*x).fold(0, |accum, x| accum + x)
}

pub fn difference(n: usize) -> usize {
    square_of_sum(n) - sum_of_squares(n)
}
