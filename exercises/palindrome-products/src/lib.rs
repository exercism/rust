#[derive(Debug, PartialEq, Eq)]
pub struct Palindrome {
    // implement your palindrome type here
}

impl Palindrome {
    pub fn new(a: u64, b: u64) -> Palindrome {
        unimplemented!("create a palindrome with factors ({}, {})", a, b)
    }

    pub fn value(&self) -> u64 {
        unimplemented!("return the value of this palindrome")
    }

    pub fn insert(&mut self, a: u64, b: u64) {
        unimplemented!("insert new factors ({}, {}) into this palindrome", a, b)
    }
}

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    unimplemented!(
        "Find the min and max palindromic numbers which are products of numbers in the inclusive range ({}..{})",
        min,
        max
    )
}
