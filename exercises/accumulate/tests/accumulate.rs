extern crate accumulate;

use accumulate::map;

fn square(x: i32) -> i32 {
    x * x
}

#[test]
fn func_single() {
    let input = vec![2];
    let expected = vec![4];
    assert_eq!(expected, map(input, square));
}

#[test]
#[ignore]
fn func_multi() {
    let input = vec![2, 3, 4, 5];
    let expected = vec![4, 9, 16, 25];
    assert_eq!(expected, map(input, square));
}

#[test]
#[ignore]
fn closure() {
    let input = vec![2, 3, 4, 5];
    let expected = vec![4, 9, 16, 25];
    assert_eq!(expected, map(input, |x| x * x));
}

#[test]
#[ignore]
fn closure_floats() {
    let input = vec![2.0, 3.0, 4.0, 5.0];
    let expected = vec![4.0, 9.0, 16.0, 25.0];
    assert_eq!(expected, map(input, |x| x * x));
}

#[test]
#[ignore]
fn strings() {
    let input = vec!["1".to_string(), "2".into(), "3".into()];
    let expected = vec!["11".to_string(), "22".into(), "33".into()];
    assert_eq!(expected, map(input, |s| s.repeat(2)));
}

#[test]
#[ignore]
fn change_in_type() {
    let input: Vec<&str> = vec!["1", "2", "3"];
    let expected: Vec<String> = vec!["1".into(), "2".into(), "3".into()];
    assert_eq!(expected, map(input, |s| s.to_string()));
}

#[test]
#[ignore]
fn mutating_closure() {
    let mut counter = 0;
    let input = vec![-2, 3, 4, -5];
    let expected = vec![2, 3, 4, 5];
    let result = map(input, |x: i64| {
        counter += 1;
        x.abs()
    });
    assert_eq!(expected, result);
    assert_eq!(4, counter);
}

#[test]
#[ignore]
fn minimal_bounds_on_input_and_output() {
    // must be able to accept arbitrary input and output types
    struct Foo;
    struct Bar;
    map(vec![Foo], |_| Bar);
}
