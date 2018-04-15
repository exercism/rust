#[derive(Debug, PartialEq)]
pub enum Error{
    Empty,
    RangeFailure,
}

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub struct ReturnVals{
    pub result: i32,
    pub factors: Vec<(i32, i32)>,
}

pub fn get_smallest_palindrome_product(min: i32, max: i32) -> Result<ReturnVals, Error>{
    if min > max{
        return Err(Error::RangeFailure);
    }

    let palindrome: Result<i32, Error> = get_smallest_palindrome(min, max);
    if palindrome.is_err(){
        return Err(palindrome.unwrap_err());
    }
    let p = palindrome.unwrap();
    Ok(ReturnVals{
        result: p,
        factors: get_factors(p, min, max),
    })
}

pub fn get_largest_palindrome_product(min: i32, max: i32) -> Result<ReturnVals, Error>{
    if min > max{
        return Err(Error::RangeFailure);
    }

    let palindrome: Result<i32, Error> = get_largest_palindrome(min, max);
    if palindrome.is_err(){
        return Err(palindrome.unwrap_err());
    }
    let p = palindrome.unwrap();
    Ok(ReturnVals{
        result: p,
        factors: get_factors(p, min, max),
    })
}

fn get_factors(n: i32, min: i32, max:i32)-> Vec<(i32, i32)>{
    let mut factors = Vec::new();

    for number in min .. max{
        let div = n/number;
        if n % number == 0 &&  div <= max && div >= min && !factors.contains(&(div, number)){
            factors.push((number, n/number));
        }
    }
    factors
}

fn get_smallest_palindrome(min: i32, max:i32)-> Result<i32, Error>{
    let l:Vec<i32> = (min*min .. max*max).collect();
        let filtered: Vec<i32> = l.iter()
        .cloned()
        .filter(|n| is_palindrome(*n) && has_factors(*n, min, max))
        .collect::<Vec<i32>>();
        if filtered.is_empty(){
            return Err(Error::Empty);
        } else{
            Ok(*filtered.iter().min().unwrap())
        }
}

fn get_largest_palindrome(min: i32, max:i32)-> Result<i32, Error>{
        let l:Vec<i32> = (min*min .. max*max).collect();
        let filtered: Vec<i32> = l.iter()
        .cloned()
        .filter(|n| is_palindrome(*n) && has_factors(*n, min, max))
        .collect::<Vec<i32>>();
        if filtered.is_empty(){
            return Err(Error::Empty);
        } else{
            Ok(*filtered.iter().max().unwrap())
        }
        
}
 
fn has_factors(n: i32, min:i32, max:i32)->bool{
    let fac = get_factors(n, min, max);
    !fac.is_empty()
}

fn is_palindrome(s: i32)->bool{
    let s1 = s.to_string();
    let s2 = s1.chars().rev().collect::<String>();

    s1 == s2
}