use prime_factors::factors;

#[test]
fn test_no_factors() { assert_eq!(vec![], factors(1)); }

#[test]
#[ignore]
fn test_prime_number() { assert_eq!(vec![2], factors(2)); }

#[test]
#[ignore]
fn test_square_of_a_prime() { assert_eq!(vec![3, 3], factors(9)); }

#[test]
#[ignore]
fn test_cube_of_a_prime() { assert_eq!(vec![2, 2, 2], factors(8)); }

#[test]
#[ignore]
fn test_product_of_primes_and_non_primes() { assert_eq!(vec![2, 2, 3], factors(12)); }

#[test]
#[ignore]
fn test_product_of_primes() { assert_eq!(vec![5, 17, 23, 461], factors(901255)); }

#[test]
#[ignore]
fn test_factors_include_large_prime() { assert_eq!(vec![11, 9539, 894119], factors(93819012551)); }
