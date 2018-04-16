extern crate palindrome_products;
use palindrome_products::*;

enum GET{
    Smallest,
    Largest,
}

fn test(e: GET, (min, max): (i32, i32))->Result<ReturnVals, Error>{
    match e{
        GET::Smallest => return get_smallest_palindrome_product(min, max),
        GET::Largest => return get_largest_palindrome_product(min, max),
    }
}
#[test]
fn smallest_palindrome_single_digits(){
    assert_eq!(test(GET::Smallest, (1,9)), Ok(ReturnVals{result: 1, factors: vec!((1,1))}));
}

#[test]
#[ignore]
fn largest_palindrome_single_digits(){
    assert_eq!(test(GET::Largest, (1,9)), Ok(ReturnVals{result: 9, factors: vec!((1,9), (3,3))}));
}

#[test]
#[ignore]
fn smallest_palindrome_double_digits(){
    assert_eq!(test(GET::Smallest, (10,99)), Ok(ReturnVals{result: 121, factors: vec!((11,11))}))
}

#[test]
#[ignore]
fn largest_palindrome_double_digits(){
    assert_eq!(test(GET::Largest, (10,99)), Ok(ReturnVals{result: 9009, factors: vec!((91,99))}))
}

#[test]
#[ignore]
fn smallest_palindrome_triple_digits(){
    assert_eq!(test(GET::Smallest, (100,999)), Ok(ReturnVals{result: 10201, factors: vec!((101,101))}))
}

#[test]
#[ignore]
fn largest_palindrome_triple_digits(){
    assert_eq!(test(GET::Largest, (100,999)), Ok(ReturnVals{result: 906609, factors: vec!((913,993))}))
}

#[test]
#[ignore]
fn smallest_palindrome_four_digits(){
    assert_eq!(test(GET::Smallest, (1000,9999)), Ok(ReturnVals{result: 1002001, factors: vec!((1001,1001))}))
}

#[test]
#[ignore]
fn largest_palindrome_four_digits(){
    assert_eq!(test(GET::Largest, (1000,9999)), Ok(ReturnVals{result: 99000099, factors: vec!((9901,9999))}))
}

#[test]
#[ignore]
fn empty_result_for_smallest_palindrome(){
    assert_eq!(test(GET::Smallest, (1002,1003)), Err(Error::Empty));
}

#[test]
#[ignore]
fn empty_result_for_largest_palindrome(){
    assert_eq!(test(GET::Largest, (15,15)), Err(Error::Empty));
}

#[test]
#[ignore]
fn error_smallest_palindrome_when_min_bt_max(){
    assert_eq!(test(GET::Smallest, (1000,1)), Err(Error::RangeFailure));
}

#[test]
#[ignore]
fn error_largest_palindrome_when_min_bt_max(){
    assert_eq!(test(GET::Largest, (2,1)), Err(Error::RangeFailure));
}
