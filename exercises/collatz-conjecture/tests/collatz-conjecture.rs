extern crate collatz_conjecture;

use collatz_conjecture::*;

#[test]
fn test_1() {
    assert_eq!(Ok(0), collatz(1));
}

#[test]
#[ignore]
fn test_16() {
    assert_eq!(Ok(4), collatz(16));
}

#[test]
#[ignore]
fn test_12() {
    assert_eq!(Ok(9), collatz(12));
}

#[test]
#[ignore]
fn test_1000000() {
    assert_eq!(Ok(152), collatz(1000000));
}

#[test]
#[ignore]
fn test_0() {
    assert!(collatz(0).is_err());
}
