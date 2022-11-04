#![feature(test)]
extern crate test;
use test::Bencher;

fn main() {
    println!("Hello, world!");
}

pub fn square_bit(s: u32) -> u64 {
    if (s < 1) | (s > 64) {
        panic!("Square must be between 1 and 64");
    }
    1 << (s - 1)
}

pub fn total_bit() -> u64 {
    ((1_u128 << 64) - 1) as u64
}

pub fn square_pow(s: u32) -> u64 {
    if (s < 1) | (s > 64) {
        panic!("Square must be between 1 and 64");
    }
    2u64.pow(s - 1)
}

pub fn total_pow_u128() -> u64 {
    (2_u128.pow(64) - 1) as u64
}

pub fn total_pow_fold() -> u64 {
    (1_u32..=64_u32).fold(0u64, |total, num| total + square_pow(num))
}

pub fn total_pow_for() -> u64 {
    let mut accum = 0;
    for i in 1..=64 {
        accum += square_pow(i)
    }
    accum
}

#[bench]
fn test_square_bit(b: &mut Bencher) {
    b.iter(|| square_bit(64));
}

#[bench]
fn test_total_bit(b: &mut Bencher) {
    b.iter(|| total_bit());
}

#[bench]
fn test_square_pow(b: &mut Bencher) {
    b.iter(|| square_pow(64));
}

#[bench]
fn test_total_pow_u128(b: &mut Bencher) {
    b.iter(|| total_pow_u128());
}

#[bench]
fn test_total_pow_fold(b: &mut Bencher) {
    b.iter(|| total_pow_fold());
}

#[bench]
fn test_total_pow_for(b: &mut Bencher) {
    b.iter(|| total_pow_for());
}
