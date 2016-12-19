use std::collections::BTreeSet;

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut multiples: BTreeSet<u32> = BTreeSet::new();

    for &f in factors {
        let mut multiplier = 2;
        let mut x = f;
        while x < limit {
            multiples.insert(x);
            x = f * multiplier;
            multiplier += 1;
        }
    }

    multiples.iter().sum()
}
