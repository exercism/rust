use nth_prime as np;

#[test]
fn test_first_prime() {
    assert_eq!(np::nth(1), 2);
}

#[test]
#[ignore]
fn test_second_prime() {
    assert_eq!(np::nth(2), 3);
}

#[test]
#[ignore]
fn test_sixth_prime() {
    assert_eq!(np::nth(6), 13);
}

#[test]
#[ignore]
fn test_big_prime() {
    assert_eq!(np::nth(10001), 104743);
}
