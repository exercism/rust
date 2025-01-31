use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Palindrome {
    // TODO
}

impl Palindrome {
    pub fn value(&self) -> u64 {
        todo!("return the value of the palindrome")
    }

    pub fn into_factors(self) -> HashSet<(u64, u64)> {
        todo!("return the set of factors of the palindrome")
    }
}

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    todo!(
        "returns the minimum and maximum number of palindromes of the products of two factors in the range {min} to {max}"
    );
}
