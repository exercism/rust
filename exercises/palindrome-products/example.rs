pub type Palindrome = u64;
pub fn get_palindrome_products(min: u64, max: u64) -> Vec<Palindrome>{
    let mut palindromes:Vec<u64> = Vec::new();
    for i in min..max+1{
        for j in min..max+1{
            if is_palindrome(i*j){
                palindromes.push(i*j);
            }
        }
    }
    palindromes
}

pub fn min(palindromes: Vec<Palindrome>)->Option<Palindrome>{
    if palindromes.is_empty(){
        return None;
    }
    palindromes.iter().cloned().min()
}

pub fn max(palindromes: Vec<Palindrome>)->Option<Palindrome>{
    if palindromes.is_empty(){
        return None;
    }
    palindromes.iter().cloned().max()
}

fn is_palindrome(s: u64)->bool{
    let s1 = s.to_string();
    let s2 = s1.chars().rev().collect::<String>();

    s1 == s2
}