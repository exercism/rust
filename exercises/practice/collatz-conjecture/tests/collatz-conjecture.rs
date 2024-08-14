use collatz_conjecture::*;

#[test]
fn zero_steps_for_one() {
    let output = collatz(1);
    let expected = Some(0);
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn divide_if_even() {
    let output = collatz(16);
    let expected = Some(4);
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn even_and_odd_steps() {
    let output = collatz(12);
    let expected = Some(9);
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn large_number_of_even_and_odd_steps() {
    let output = collatz(1_000_000);
    let expected = Some(152);
    assert_eq!(output, expected);
}

#[test]
#[ignore]
fn zero_is_an_error() {
    let output = collatz(0);
    let expected = None;
    assert_eq!(output, expected);
}
