use std::collections::HashMap;
use std::collections::hashmap::{Occupied, Vacant};

pub fn count(nucleotide: char, input: &str) -> uint {
    input.chars().filter(|&c| c == nucleotide).count()
}

pub fn nucleotide_counts(input: &str) -> HashMap<char, uint> {
    let mut map: HashMap<char, uint> = "ACGT".chars().map(|c| (c, 0)).collect();
    for nucleotide in input.chars() {
        match map.entry(nucleotide) {
            Vacant(entry) => entry.set(1),
            Occupied(mut entry) => {
                *entry.get_mut() += 1;
                entry.into_mut()
            }
        };
    }
    map
}
