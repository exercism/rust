extern crate rand;

use rand::distributions::{IndependentSample, Range};

/// Right-to-left modular exponentiation implementation
/// For more information see https://en.wikipedia.org/wiki/Modular_exponentiation
fn modular_exponentiation(base: u64, exponent: u64, modulus: u64) -> u64 {
    let mut result = 1;

    // avoid overflow on multiplication
    let mut e = exponent as u128;
    let m = modulus as u128;
    let mut b = (base % modulus) as u128;

    while e > 0 {
        if (e & 1) == 1 {
            result = (result * b) % m;
        }

        e >>= 1;

        b = (b * b) % m;
    }

    result as u64
}

pub fn private_key(p: u64) -> u64 {
    Range::new(2, p).ind_sample(&mut rand::thread_rng())
}

pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    modular_exponentiation(g, a, p)
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    modular_exponentiation(b_pub, a, p)
}
