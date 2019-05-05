use nth_prime as np;

#[test]
fn test_first_prime() { assert_eq!(2, np::nth(0)); }

#[test]
#[ignore]
fn test_second_prime() { assert_eq!(3, np::nth(1)); }

#[test]
#[ignore]
fn test_sixth_prime() { assert_eq!(13, np::nth(5)); }

#[test]
#[ignore]
fn test_big_prime() { assert_eq!(104743, np::nth(10000)); }
