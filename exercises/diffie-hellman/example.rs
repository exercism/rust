extern crate rand;

use rand::distributions::{IndependentSample, Range};

pub fn private_key(p: &u64) -> u64 {
    Range::new(2, *p).ind_sample(&mut rand::thread_rng())
}

pub fn public_key(p: &u64, g: &u64, a: &u64) -> u64 {
    assert_ne!((*g).checked_mul((*a)), None);
    ((*g) * (*a)) % (*p)
}

pub fn secret(p: &u64, b_pub: &u64, a: &u64) -> u64 {
    assert_ne!((*b_pub).checked_mul((*a)), None);
    ((*b_pub) * (*a)) % (*p)
}
