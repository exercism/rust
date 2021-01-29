use collatz_conjecture::*;

#[test]
fn test_1() {
    assert_eq!(Some(0), collatz(1));
}

#[test]
#[ignore]
fn test_16() {
    assert_eq!(Some(4), collatz(16));
}

#[test]
#[ignore]
fn test_12() {
    assert_eq!(Some(9), collatz(12));
}

#[test]
#[ignore]
fn test_1000000() {
    assert_eq!(Some(152), collatz(1_000_000));
}

#[test]
#[ignore]
fn test_0() {
    assert_eq!(None, collatz(0));
}
