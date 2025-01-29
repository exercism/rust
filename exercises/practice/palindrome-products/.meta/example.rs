use std::cmp::Ordering;
use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct Palindrome {
    value: u64,
    factors: HashSet<(u64, u64)>,
}

impl Palindrome {
    pub fn new(value: u64, first_factors: (u64, u64)) -> Palindrome {
        Self {
            value,
            factors: HashSet::from([first_factors]),
        }
    }

    pub fn add_factor(&mut self, factor: (u64, u64)) -> bool {
        self.factors.insert(factor)
    }

    pub fn value(&self) -> u64 {
        self.value
    }

    pub fn factors(&self) -> &HashSet<(u64, u64)> {
        &self.factors
    }
}

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    let mut pmin: Option<Palindrome> = None;
    let mut pmax: Option<Palindrome> = None;
    for i in min..=max {
        for j in i..=max {
            let p = i * j;
            if is_palindrome(p) {
                pmin = match pmin.as_ref().map(|prev| prev.value.cmp(&p)) {
                    Some(Ordering::Less) => pmin,
                    Some(Ordering::Equal) => {
                        if i <= j {
                            pmin.as_mut().unwrap().add_factor((i, j));
                        }
                        pmin
                    }
                    Some(Ordering::Greater) | None => Some(Palindrome::new(p, (i, j))),
                };

                pmax = match pmax.as_ref().map(|prev| prev.value.cmp(&p)) {
                    Some(Ordering::Greater) => pmax,
                    Some(Ordering::Equal) => {
                        if i <= j {
                            pmax.as_mut().unwrap().add_factor((i, j));
                        }
                        pmax
                    }
                    Some(Ordering::Less) | None => Some(Palindrome::new(p, (i, j))),
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
