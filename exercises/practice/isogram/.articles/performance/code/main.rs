#![feature(test)]
extern crate test;
use test::Bencher;

fn main() {
    println!("Hello, world!");
}

use std::collections::HashSet;

pub fn check_hash(candidate: &str) -> bool {
    let mut hs = HashSet::new();
    candidate
        .bytes()
        .filter(|c| c.is_ascii_alphabetic())
        .map(|c| c.to_ascii_lowercase())
        .all(|c| hs.insert(c))
}

const A_LCASE: u8 = 97;
const Z_LCASE: u8 = 122;
const A_UCASE: u8 = 65;
const Z_UCASE: u8 = 90;

pub fn check_bits(candidate: &str) -> bool {
    let mut letter_flags: u32 = 0;

    for letter in candidate.bytes() {
        if letter >= A_LCASE && letter <= Z_LCASE {
            if letter_flags & (1 << (letter - A_LCASE)) != 0 {
                return false;
            } else {
                letter_flags |= 1 << (letter - A_LCASE);
            }
        } else if letter >= A_UCASE && letter <= Z_UCASE {
            if letter_flags & (1 << (letter - A_UCASE)) != 0 {
                return false;
            } else {
                letter_flags |= 1 << (letter - A_UCASE);
            }
        }
    }
    return true;
}

pub fn check_bits_func(candidate: &str) -> bool {
    candidate
        .bytes()
        .filter_map(|c| {
            c.is_ascii_alphabetic()
                .then(|| 1u32 << (c.to_ascii_lowercase() - A_LCASE))
        })
        .try_fold(0u32, |ltr_flags, ltr| {
            (ltr_flags & ltr == 0).then(|| ltr_flags | ltr)
        })
        .is_some()
}

pub fn check_hash_filtermap(candidate: &str) -> bool {
    let mut hs = HashSet::new();
    candidate
        .bytes()
        .filter_map(|c| c.is_ascii_alphabetic().then(|| c.to_ascii_lowercase()))
        .all(|c| hs.insert(c))
}

pub fn check_bits_func_filter_map(candidate: &str) -> bool {
    candidate
        .bytes()
        .filter(|c| c.is_ascii_alphabetic())
        .map(|c| 1u32 << (c.to_ascii_lowercase() - A_LCASE))
        .try_fold(0u32, |ltr_flags, ltr| {
            (ltr_flags & ltr == 0).then(|| ltr_flags | ltr)
        })
        .is_some()
}

pub fn check_hash_unicode(candidate: &str) -> bool {
    let mut hs = std::collections::HashSet::new();
    candidate
        .to_lowercase()
        .chars()
        .filter(|c| c.is_alphabetic())
        .all(|c| hs.insert(c))
}

#[bench]
fn test_check_hash(b: &mut Bencher) {
    b.iter(|| check_hash("thumbscrew-japingly"));
}

#[bench]
fn test_check_bits(b: &mut Bencher) {
    b.iter(|| check_bits("thumbscrew-japingly"));
}

#[bench]
fn test_check_bits_func(b: &mut Bencher) {
    b.iter(|| check_bits_func("thumbscrew-japingly"));
}

#[bench]
fn test_check_hash_filtermap(b: &mut Bencher) {
    b.iter(|| check_hash_filtermap("thumbscrew-japingly"));
}

#[bench]
fn test_check_bits_func_filter_map(b: &mut Bencher) {
    b.iter(|| check_bits_func_filter_map("thumbscrew-japingly"));
}

#[bench]
fn test_check_hash_unicode(b: &mut Bencher) {
    b.iter(|| check_hash_unicode("thumbscrew-japingly"));
}
