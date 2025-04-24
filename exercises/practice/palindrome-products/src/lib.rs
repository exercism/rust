use std::collections::{HashMap, HashSet};
use itertools::Itertools; 

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Palindrome {
    value: u64,
    factors: HashSet<(u64, u64)>
}

impl Palindrome {
    pub fn new(value: u64, factors: HashSet<(u64, u64)>) -> Self {
        Self {
            value,
            factors
        }
    }

    pub fn value(&self) -> u64 {
        self.value
    }

    pub fn into_factors(self) -> HashSet<(u64, u64)> {
        self.factors
    }
}

// Can be done with just two palindromes (palindrome_min and palindrome_max) that can
// be updated. Rather than with a full HashMap<u64, HashSet<(u64, u64)>>
pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    let mut palindromes: HashMap<u64, HashSet<(u64, u64)>> = HashMap::new();

    let palindrome_factors:Vec<(u64, u64)> = (min..=max)
    .combinations_with_replacement(2)
    .filter(|pair| is_palindrome(pair[0] * pair[1]))
    .map(|pair| (pair[0], pair[1])).collect();

    println!("{:?}",palindrome_factors);

    if palindrome_factors.is_empty() {
        return None;
    }

    for pair in palindrome_factors {
        palindromes.entry(pair.0*pair.1).or_insert(HashSet::new()).insert(pair);
    }

    println!("{:?}",palindromes);

    let min_key = palindromes.keys().min().unwrap();
    let max_key = palindromes.keys().max().unwrap();

    Some((Palindrome::new(*min_key, palindromes.get(min_key).unwrap().clone()),
    Palindrome::new(*max_key, palindromes.get(max_key).unwrap().clone())))

}

fn is_palindrome(n:u64) -> bool {
    let mut input = n;
    let mut reversed = 0;

    while input > 0 {
        reversed = reversed*10 + (input%10);
        input /= 10;
    }

    reversed == n
}
