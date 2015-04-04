use std::collections::HashMap;
use std::collections::hash_map::Entry;

pub fn count(nucleotide: char, input: &str) -> usize {
    input.chars().filter(|&c| c == nucleotide).count()
}

pub fn nucleotide_counts(input: &str) -> HashMap<char, usize> {
    let mut map: HashMap<char, usize> = "ACGT".chars().map(|c| (c, 0)).collect();
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
