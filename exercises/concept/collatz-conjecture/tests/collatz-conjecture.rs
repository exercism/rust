use collatz_conjecture::{collatz_count, collatz_next};

#[test]
fn test_odd() {
    assert_eq!(collatz_next(5), 16);
}

#[test]
#[ignore]
fn test_even() {
    assert_eq!(collatz_next(16), 8);
}

#[test]
#[ignore]
fn test_1() {
    assert_eq!(collatz_count(1), 0);
}

#[test]
#[ignore]
fn test_16() {
    assert_eq!(collatz_count(16), 4);
}

#[test]
#[ignore]
fn test_12() {
    assert_eq!(collatz_count(12), 9);
}

#[test]
#[ignore]
fn test_1000000() {
    assert_eq!(collatz_count(1_000_000), 152);
}
