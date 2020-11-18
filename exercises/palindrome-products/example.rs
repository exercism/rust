#[derive(Debug, PartialEq, Eq)]
pub struct Palindrome {
    pub factors: Vec<(u64, u64)>,
}

impl Palindrome {
    /// Create a palindrome with the given factors
    pub fn new(mut a: u64, mut b: u64) -> Palindrome {
        if a > b {
            std::mem::swap(&mut a, &mut b);
        }
        Palindrome {
            factors: vec![(a, b)],
        }
    }

    /// Return the palindrome's value
    pub fn value(&self) -> u64 {
        // this could in theory panic with a bounds error, but the length of
        // self.factors is known to start at 1 and is only ever increased
        self.factors[0].0 * self.factors[0].1
    }

    /// Insert a new set of factors into an existing palindrome
    pub fn insert(&mut self, mut a: u64, mut b: u64) {
        if a > b {
            std::mem::swap(&mut a, &mut b);
        }
        self.factors.push((a, b));
        self.factors.sort_unstable();
        self.factors.dedup();
    }
}

/// return the (min, max) palindrome pair comprised of the products of numbers in the input range
pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    let mut result = None;

    for a in min..=max {
        for b in min..=a {
            if is_palindrome(a * b) {
                result = match result {
                    None => Some((Palindrome::new(a, b), Palindrome::new(a, b))),
                    Some((mut minp, mut maxp)) => {
                        match (a * b).cmp(&minp.value()) {
                            std::cmp::Ordering::Greater => {}
                            std::cmp::Ordering::Less => minp = Palindrome::new(a, b),
                            std::cmp::Ordering::Equal => minp.insert(a, b),
                        }
                        match (a * b).cmp(&maxp.value()) {
                            std::cmp::Ordering::Less => {}
                            std::cmp::Ordering::Greater => maxp = Palindrome::new(a, b),
                            std::cmp::Ordering::Equal => maxp.insert(a, b),
                        }
                        Some((minp, maxp))
                    }
                };
            }
        }
    }

    result
}

#[inline]
pub fn is_palindrome(n: u64) -> bool {
    let s = n.to_string().into_bytes();
    s.iter().zip(s.iter().rev()).all(|(a, b)| a == b)
}
