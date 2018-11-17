pub type Palindrome = u64;
pub fn get_palindrome_products(min: u64, max: u64) -> Vec<Palindrome> {
    let mut palindromes: Vec<u64> = Vec::new();
    for i in min..max + 1 {
        for j in min..max + 1 {
            if is_palindrome(i * j) {
                palindromes.push(i * j);
            }
        }
    }
    palindromes
}

pub fn min(palindromes: &[Palindrome]) -> Option<Palindrome> {
    palindromes.iter().min().cloned()
}

pub fn max(palindromes: &[Palindrome]) -> Option<Palindrome> {
    palindromes.iter().max().cloned()
}

fn is_palindrome(n: u64) -> bool {
    let s = n.to_string().into_bytes();
    s.iter().zip(s.iter().rev()).all(|(c1, c2)| c1 == c2)
}
