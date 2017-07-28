extern crate nth_prime as np;

#[test]
fn test_first_prime() {
    assert_eq!(np::nth(1), Ok(2));
}

#[test]
#[ignore]
fn test_second_prime() {
    assert_eq!(np::nth(2), Ok(3));
}

#[test]
#[ignore]
fn test_sixth_prime() {
    assert_eq!(np::nth(6), Ok(13));
}

#[test]
#[ignore]
fn test_big_prime() {
    assert_eq!(np::nth(10001), Ok(104743));
}

#[test]
#[ignore]
fn test_zeroth_prime() {
    assert!(np::nth(0).is_err());
}
