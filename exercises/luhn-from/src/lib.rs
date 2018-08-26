pub struct Luhn;

impl Luhn {
    pub fn is_valid(&self) -> bool {
        unimplemented!("Determine if the current Luhn struct contains a valid credit card number.");
    }
}

/// Here is the example of how the From trait could be implemented
/// for the &str type. Naturally, you can implement this trait
/// by hand for the every other type presented in the test suite,
/// but your solution will fail if a new type is presented.
/// Perhaps there exists a better solution for this problem?
impl<'a> From<&'a str> for Luhn {
    fn from(input: &'a str) -> Self {
        unimplemented!("From the given input '{}' create a new Luhn struct.", input);
    }
}
