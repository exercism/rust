pub trait Luhn {
    fn valid_luhn(&self) -> bool;
}

/// Here is the example of how to implement custom Luhn trait
/// for the &str type. Naturally, you can implement this trait
/// by hand for the every other type presented in the test suite,
/// but your solution will fail if a new type is presented.
/// Perhaps there exists a better solution for this problem?
impl<'a> Luhn for &'a str {
    fn valid_luhn(&self) -> bool {
        unimplemented!("Determine if '{}' is a valid credit card number.", self);
    }
}
