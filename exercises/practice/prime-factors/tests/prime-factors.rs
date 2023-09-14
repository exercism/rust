use prime_factors::factors;

#[test]
fn no_factors() {
    assert_eq!(factors(1), vec![]);
}

#[test]
#[ignore]
fn prime_number() {
    assert_eq!(factors(2), vec![2]);
}

#[test]
#[ignore]
fn square_of_a_prime() {
    assert_eq!(factors(9), vec![3, 3]);
}

#[test]
#[ignore]
fn cube_of_a_prime() {
    assert_eq!(factors(8), vec![2, 2, 2]);
}

#[test]
#[ignore]
fn product_of_primes_and_non_primes() {
    assert_eq!(factors(12), vec![2, 2, 3]);
}

#[test]
#[ignore]
fn product_of_primes() {
    assert_eq!(factors(901_255), vec![5, 17, 23, 461]);
}

#[test]
#[ignore]
fn factors_include_large_prime() {
    assert_eq!(factors(93_819_012_551), vec![11, 9539, 894_119]);
}
