pub struct Luhn(String);

impl Luhn {
    pub fn is_valid(&self) -> bool {

        self.0
        .chars()
        .filter(|c| !c.is_whitespace())
        .rev()
        .try_fold((0, 0), |(i, checksum), c| {
            c.to_digit(10) // This (combined with the try_fold) force a return to false if c is not a digit
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
            .map(|d| {
                (i+1, checksum+d)
            })
        })
        .is_some_and(|(len, checksum)| len > 1 && checksum % 10 == 0)
    }
}

/// Here is the example of how the From trait could be implemented
/// for the &str type. Naturally, you can implement this trait
/// by hand for every other type presented in the test suite,
/// but your solution will fail if a new type is presented.
/// Perhaps there exists a better solution for this problem?
impl<T: ToString> From<T> for Luhn {
    // works because String also have a `to_string()` method
    fn from(input: T) -> Self {
        Self(
            input.to_string()
        )
    }
}
