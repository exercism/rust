/// `Palindrome` is a newtype which only exists when the contained value is a palindrome number in base ten.
///
/// A struct with a single field which is used to constrain behavior like this is called a "newtype", and its use is
/// often referred to as the "newtype pattern". This is a fairly common pattern in Rust.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
pub struct Palindrome(u64);

impl Palindrome {
    /// Create a `Palindrome` only if `value` is in fact a palindrome when represented in base ten. Otherwise, `None`.
    pub fn new(value: u64) -> Option<Palindrome> {
        is_palindrome(value).then(move || Palindrome(value))
    }

    /// Get the value of this palindrome.
    pub fn into_inner(self) -> u64 {
        self.0
    }
}

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    let mut pmin: Option<Palindrome> = None;
    let mut pmax: Option<Palindrome> = None;
    for i in min..=max {
        for j in i..=max {
            if let Some(palindrome) = Palindrome::new(i * j) {
                pmin = match pmin {
                    None => Some(palindrome),
                    Some(prev) => Some(prev.min(palindrome)),
                };
                pmax = match pmax {
                    None => Some(palindrome),
                    Some(prev) => Some(prev.max(palindrome)),
                };
            }
        }
    }

    pmin.zip(pmax)
}

#[inline]
pub fn is_palindrome(n: u64) -> bool {
    let s = n.to_string().into_bytes();
    s.iter().zip(s.iter().rev()).all(|(a, b)| a == b)
}
