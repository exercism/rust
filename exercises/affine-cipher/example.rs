const MODULUS: i32 = 26;

/// An error type for indicating problems with the input
#[derive(Debug, Eq, PartialEq)]
pub enum AffineCipherError {
    NotCoprime(i32),
}

/// Encodes the plaintext using the affine cipher with key (`a`, `b`). Note that, rather than
/// returning a return code, the more common convention in Rust is to return a `Result`.
pub fn encode(plaintext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    // Reject the key if `a` and `MODULUS` aren't coprime.
    match modular_multiplicative_inverse(a) {
        Some(_) => Ok(plaintext
            .to_lowercase()
            .chars()
            .filter(|&ch| ch.is_ascii_alphanumeric())
            .map(|ch| encode_char(ch, a, b))
            .collect::<Vec<char>>()
            .chunks(5)
            .map(|slice| slice.iter().cloned().collect::<String>())
            .collect::<Vec<String>>()
            .join(" ")),
        None => Err(AffineCipherError::NotCoprime(a)),
    }
}

/// Decodes the ciphertext using the affine cipher with key (`a`, `b`). Note that, rather than
/// returning a return code, the more common convention in Rust is to return a `Result`.
pub fn decode(ciphertext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    // Reject the key if `a` and `MODULUS` aren't coprime.
    match modular_multiplicative_inverse(a) {
        Some(inv) => Ok(ciphertext
            .to_lowercase()
            .chars()
            .filter(|&ch| ch.is_ascii_alphanumeric())
            .map(|ch| decode_char(ch, inv, b))
            .collect()),
        None => Err(AffineCipherError::NotCoprime(a)),
    }
}

/// Encodes a single char with the key (`a`, `b`). The key is assumed to be valid (i.e. `a` should
/// be coprime to 26).
fn encode_char(ch: char, a: i32, b: i32) -> char {
    if ch.is_digit(10) {
        ch
    } else {
        let index = (ch as i32) - ('a' as i32);
        let encoded = (a * index + b).rem_euclid(MODULUS) + 'a' as i32;
        encoded as u8 as char
    }
}

/// Decodes a single char using `inv` (the modular multiplicative inverse of `a`) and `b`.
fn decode_char(ch: char, inv: i32, b: i32) -> char {
    if ch.is_digit(10) {
        ch
    } else {
        let index = (ch as i32) - ('a' as i32);
        let decoded = (inv * (index - b)).rem_euclid(MODULUS) + 'a' as i32;
        decoded as u8 as char
    }
}

/// Calculates the modular multiplicative inverse using the extended Euclidean algorithm.
/// See https://en.wikipedia.org/wiki/Extended_Euclidean_algorithm for details.
fn modular_multiplicative_inverse(a: i32) -> Option<i32> {
    // `rs` corresponds to the `r_i` sequence and `ts` corresponds to the `t_i` sequence. We omit
    // `s_i` since we don't need it to calculate the MMI.
    let mut rs = (MODULUS, a.rem_euclid(MODULUS));
    let mut ts = (0, 1);

    while rs.1 != 0 {
        let q = rs.0.div_euclid(rs.1);

        rs = (rs.1, rs.0 - q * rs.1);
        ts = (ts.1, ts.0 - q * ts.1);
    }

    // `rs.0` gives the GCD. This must be 1 for an inverse to exist.
    if rs.0 == 1 {
        // ts.0 gives a number such that (s * 26 + t * a) % 26 == 1. Since (s * 26) % 26 == 0,
        // we can further reduce this to (t * a) % 26 == 1. In other words, t is the MMI of a.
        Some(ts.0 as i32)
    } else {
        None
    }
}
