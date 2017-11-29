extern crate rand;

use rand::distributions::{IndependentSample, Range};

pub fn private_key(p: u64) -> u64 {
    Range::new(1, p).ind_sample(&mut rand::thread_rng())
}

pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    g.pow(a as u32) % p
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    b_pub.pow(a as u32) % p
}
