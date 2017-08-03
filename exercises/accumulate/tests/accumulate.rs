extern crate accumulate;

use accumulate::{map_function, map_closure};

fn square(x: i32) -> i32 {
    x*x
}

fn abs_val(x: i32) -> i32 {
    if x > 0 {
        x
    } else {
        -x
    }
}

#[test]
fn test_func_square_single() {
    let input = vec![2];
    let expected = vec![4];
    assert_eq!(map_function(input, &square), expected);
}

#[test]
#[ignore]
fn test_func_square_short() {
    let input = vec![2, 3, 4, 5];
    let expected = vec![4, 9, 16, 25];
    assert_eq!(map_function(input, &square), expected);
}

#[test]
#[ignore]
fn test_func_square_long_with_neg() {
    let input = vec![2, -3, -2, 3, 4, 3, 4, 5, 100, 8, 16, 34];
    let expected = vec![4, 9, 4, 9, 16, 9, 16, 25, 10000, 64, 256, 1156];
    assert_eq!(map_function(input, &square), expected);
}

#[test]
#[ignore]
fn test_func_abs_value_with_neg() {
    let input = vec![-3];
    let expected = vec![3];
    assert_eq!(map_function(input, &abs_val), expected);
}

#[test]
#[ignore]
fn test_func_abs_value_long() {
    let input = vec![-3, 5, -10, 4, 100, -1234, 55443];
    let expected = vec![3, 5, 10, 4, 100, 1234, 55443];
    assert_eq!(map_function(input, &abs_val), expected);
}

#[test]
#[ignore]
fn test_closure_square_single() {
    let input = vec![2];
    let expected = vec![4];
    assert_eq!(map_closure(input, |x| x*x), expected);
}

#[test]
#[ignore]
fn test_closure_square_short() {
    let input = vec![2, 3, 4, 5];
    let expected = vec![4, 9, 16, 25];
    assert_eq!(map_closure(input, |x| x*x), expected);
}

#[test]
#[ignore]
fn test_closure_square_long_with_neg() {
    let input = vec![2, -3, -2, 3, 4, 3, 4, 5, 100, 8, 16, 34];
    let expected = vec![4, 9, 4, 9, 16, 9, 16, 25, 10000, 64, 256, 1156];
    assert_eq!(map_closure(input, |x| x*x), expected);
}

#[test]
#[ignore]
fn test_closure_abs_value_with_neg() {
    let input = vec![-3];
    let expected = vec![3];
    assert_eq!(map_closure(input, |x| if x > 0 {x} else {-x}), expected);
}

#[test]
#[ignore]
fn test_closure_abs_value_long() {
    let input = vec![-3, 5, -10, 4, 100, -1234, 55443];
    let expected = vec![3, 5, 10, 4, 100, 1234, 55443];
    assert_eq!(map_closure(input, |x| if x > 0 {x} else {-x}), expected);
}
