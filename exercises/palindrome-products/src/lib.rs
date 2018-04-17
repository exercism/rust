#[derive(Debug, PartialEq)]
pub enum Error{
    Empty,
    RangeFailure,
}
#[derive(Debug, PartialEq)]
pub struct ReturnVals{
    pub result: i32,
    pub factors: Vec<(i32, i32)>,
}

pub fn get_smallest_palindrome_product(min: i32, max:i32)->Result<ReturnVals, Error>{
    unimplemented!();
}
pub fn get_largest_palindrome_product(min: i32, max: i32)->Result<ReturnVals, Error>{
    unimplemented!();
}