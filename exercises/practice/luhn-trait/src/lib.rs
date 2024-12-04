pub trait Luhn {
    fn valid_luhn(&self) -> bool;
}

/// Here is the example of how to implement custom Luhn trait
/// for the &str type. Naturally, you can implement this trait
/// by hand for every other type presented in the test suite,
/// but your solution will fail if a new type is presented.
/// Perhaps there exists a better solution for this problem?
impl Luhn for &str {
    fn valid_luhn(&self) -> bool {
        todo!("Determine if '{self}' is a valid credit card number.");
    }
}
