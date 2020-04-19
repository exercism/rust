use diffie_hellman::*;

#[test]
fn test_private_key_in_range_key() {
    let primes: Vec<u64> = vec![
        5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 773, 967, 3461, 6131,
    ];
    let private_keys: Vec<u64> = primes.iter().map(|x| private_key(*x)).collect();

    for i in 0..primes.len() {
        assert!(1 < private_keys[i] && private_keys[i] < primes[i]);
    }
}

#[test]
#[ignore]
fn test_public_key_correct() {
    let p: u64 = 23;
    let g: u64 = 5;

    let private_key: u64 = 6;
    let expected: u64 = 8;

    assert_eq!(public_key(p, g, private_key), expected);
}

#[test]
#[ignore]
fn test_secret_key_correct() {
    let p: u64 = 11;

    let private_key_a = 7;
    let public_key_b = 8;
    let secret = secret(p, public_key_b, private_key_a);
    let expected = 2;

    assert_eq!(secret, expected);
}

#[test]
#[ignore]
fn test_public_key_correct_big_numbers() {
    let p: u64 = 4_294_967_299;

    let g: u64 = 8;

    let private_key: u64 = 4_294_967_296;

    let expected: u64 = 4096;

    assert_eq!(public_key(p, g, private_key), expected);
}

#[test]
#[ignore]
fn test_secret_key_correct_big_numbers() {
    let p: u64 = 4_294_967_927;

    let private_key_a = 4_294_967_300;

    let public_key_b = 843;

    let secret = secret(p, public_key_b, private_key_a);

    let expected = 1_389_354_282;

    assert_eq!(secret, expected);
}

// two biggest 64bit primes
#[cfg(feature = "big-primes")]
const PRIME_64BIT_1: u64 = 0xFFFF_FFFF_FFFF_FFC5;
#[cfg(feature = "big-primes")]
const PRIME_64BIT_2: u64 = 0xFFFF_FFFF_FFFF_FFAC;
#[cfg(feature = "big-primes")]
const PRIVATE_KEY_64BIT: u64 = 0xFFFF_FFFF_FFFF_FFC3;
#[cfg(feature = "big-primes")]
const PUBLIC_KEY_64BIT: u64 = 0xB851_EB85_1EB8_51C1;

#[test]
#[ignore]
#[cfg(feature = "big-primes")]
fn test_public_key_correct_biggest_numbers() {
    assert_eq!(
        public_key(PRIME_64BIT_1, PRIME_64BIT_2, PRIVATE_KEY_64BIT),
        PUBLIC_KEY_64BIT
    );
}

#[test]
#[ignore]
#[cfg(feature = "big-primes")]
fn test_secret_key_correct_biggest_numbers() {
    let private_key_b = 0xEFFF_FFFF_FFFF_FFC0;
    let public_key_b = public_key(PRIME_64BIT_1, PRIME_64BIT_2, private_key_b);

    let expected_b = 4_340_425_873_327_658_043;
    assert_eq!(public_key_b, expected_b);

    let expected_key = 12_669_955_479_143_291_250;

    let secret_key = secret(PRIME_64BIT_1, public_key_b, PRIVATE_KEY_64BIT);

    assert_eq!(secret_key, expected_key);

    let secret_key = secret(PRIME_64BIT_1, PUBLIC_KEY_64BIT, private_key_b);

    assert_eq!(secret_key, expected_key);
}

#[test]
#[ignore]
#[cfg(feature = "big-primes")]
fn test_changed_secret_key_biggest_numbers() {
    let private_key_a = private_key(PRIME_64BIT_1);
    let public_key_a = public_key(PRIME_64BIT_1, PRIME_64BIT_2, private_key_a);

    let private_key_b = private_key(PRIME_64BIT_1);
    let public_key_b = public_key(PRIME_64BIT_1, PRIME_64BIT_2, private_key_b);

    let secret_a = secret(PRIME_64BIT_1, public_key_b, private_key_a);
    let secret_b = secret(PRIME_64BIT_1, public_key_a, private_key_b);

    assert_eq!(secret_a, secret_b);
}

#[test]
#[ignore]
fn test_changed_secret_key() {
    let p: u64 = 13;
    let g: u64 = 11;

    let private_key_a = private_key(p);
    let private_key_b = private_key(p);

    let public_key_a = public_key(p, g, private_key_a);
    let public_key_b = public_key(p, g, private_key_b);

    // Key exchange
    let secret_a = secret(p, public_key_b, private_key_a);
    let secret_b = secret(p, public_key_a, private_key_b);

    assert_eq!(secret_a, secret_b);
}
