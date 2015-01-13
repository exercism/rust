#![allow(unstable)] // for entry

use std::collections::BTreeMap;
use std::collections::btree_map::Entry;

pub fn count(nucleotide: char, input: &str) -> usize {
    input.chars().filter(|&c| c == nucleotide).count()
}

pub fn nucleotide_counts(input: &str) -> BTreeMap<char, usize> {
    let mut map: BTreeMap<char, usize> = "ACGT".chars().map(|c| (c, 0)).collect();
    for nucleotide in input.chars() {
        match map.entry(nucleotide) {
            Entry::Vacant(view) => view.insert(1),
            Entry::Occupied(mut view) => {
                *view.get_mut() += 1;
                view.into_mut()
            }
        };
    }
    map
}
