#![feature(test)]
extern crate test;
use test::Bencher;

fn main() {
    println!("Hello, world!");
}

use std::collections::HashSet;

pub fn is_pangram_all_contains(sentence: &str) -> bool {
    let sentence_lowered = sentence.to_lowercase();
    ('a'..='z').all(|ltr| sentence_lowered.contains(ltr))
}

pub fn is_pangram_hash_is_subset(sentence: &str) -> bool {
    let all: HashSet<char> = HashSet::from_iter("abcdefghijklmnopqrstuvwxyz".chars());
    let used: HashSet<char> = HashSet::from_iter(sentence.to_lowercase().chars());
    all.is_subset(&used)
}

pub fn is_pangram_hashset_len(sentence: &str) -> bool {
    sentence
        .to_lowercase()
        .chars()
        .filter(|c| c.is_ascii_alphabetic())
        .collect::<HashSet<char>>()
        .len()
        == 26
}

const A_LCASE: u8 = 97;
const A_UCASE: u8 = 65;
const ALL_26_BITS_SET: u32 = 67108863;

pub fn is_pangram_bitfield(sentence: &str) -> bool {
    let mut letter_flags = 0;

    for letter in sentence.chars() {
        if letter >= 'a' && letter <= 'z' {
            letter_flags |= 1 << (letter as u8 - A_LCASE);
        } else if letter >= 'A' && letter <= 'Z' {
            letter_flags |= 1 << (letter as u8 - A_UCASE);
        }
    }
    letter_flags == ALL_26_BITS_SET
}

#[bench]
fn test_is_pangram_all_contains(b: &mut Bencher) {
    b.iter(|| {
        is_pangram_all_contains(
            "Victor jagt zwölf_(12) Boxkämpfer quer über den großen Sylter Deich.",
        )
    });
}

#[bench]
fn test_is_pangram_hash_is_subset(b: &mut Bencher) {
    b.iter(|| {
        is_pangram_hash_is_subset(
            "Victor jagt zwölf_(12) Boxkämpfer quer über den großen Sylter Deich.",
        )
    });
}

#[bench]
fn test_is_pangram_hashset_len(b: &mut Bencher) {
    b.iter(|| {
        is_pangram_hashset_len(
            "Victor jagt zwölf_(12) Boxkämpfer quer über den großen Sylter Deich.",
        )
    });
}

#[bench]
fn test_is_pangram_bitfield(b: &mut Bencher) {
    b.iter(|| {
        is_pangram_bitfield("Victor jagt zwölf_(12) Boxkämpfer quer über den großen Sylter Deich.")
    });
}
