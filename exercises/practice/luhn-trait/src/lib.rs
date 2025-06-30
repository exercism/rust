pub trait Luhn {
    fn valid_luhn(&self) -> bool;
}

/// Here is the example of how to implement custom Luhn trait
/// for the &str type. Naturally, you can implement this trait
/// by hand for every other type presented in the test suite,
/// but your solution will fail if a new type is presented.
/// Perhaps there exists a better solution for this problem?
impl<T: ToString> Luhn for T {
    fn valid_luhn(&self) -> bool {
        self
        .to_string()
        .chars()
        .filter(|c| !c.is_whitespace())
        .rev()
        .try_fold((0, 0), |(i, checksum), c| {
            c.to_digit(10)
            .map(|d| {
                if i % 2 == 1 {
                    d << 1
                }
                else {
                    d
                }
            })
            .map(|d| {
                if d > 9 {
                    d - 9
                }
                else {
                    d
                }
            })
            .map(|d| (i+1, checksum+d))
        })
        .is_some_and(|(len, checksum)| len > 1 && checksum % 10 == 0)
    }
}
