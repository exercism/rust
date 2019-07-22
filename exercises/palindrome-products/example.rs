pub type Palindrome = u64;

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    let mut palindromes: Vec<u64> = Vec::new();
    for i in min..=max {
        for j in min..=max {
            if is_palindrome(i * j) {
                palindromes.push(i * j);
            }
        }
    }

    palindromes.iter().cloned().fold(None, |prev, pal| match prev {
        None => Some((pal, pal)),
        Some((pmin, pmax)) => Some((if pal < pmin {pal} else {pmin}, if pal > pmax {pal} else {pmax})),
    })
}

fn is_palindrome(n: u64) -> bool {
    let s = n.to_string().into_bytes();
    s.iter().zip(s.iter().rev()).all(|(c1, c2)| c1 == c2)
}
