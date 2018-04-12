#[derive(Debug, PartialEq)]
pub enum Error{
    Empty,
    RangeFailure,
}

pub fn get_palindrome_product(min: i32, max: i32) -> Result<i32, Error>{
    if min > max{
        return Err(Error::RangeFailure);
    }
    Ok(1)
}