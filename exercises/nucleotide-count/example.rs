use std::collections::HashMap;
use std::collections::hash_map::Entry;

static VALID_NUCLEOTIDES: &'static str = "ACGT";

pub fn count(nucleotide: char, input: &str) -> Result<usize, char> {
    let valid = |x: char| { VALID_NUCLEOTIDES.contains(x) };
    if valid(nucleotide) && input.chars().all(valid) {
        Ok(input.chars().filter(|&c| c == nucleotide).count())
    } else {
        Err(nucleotide)
    }
}

pub fn nucleotide_counts(input: &str) -> Result<HashMap<char, usize>, char> {
    let mut map: HashMap<char, usize> = VALID_NUCLEOTIDES.chars().map(|c| (c, 0)).collect();
    for nucleotide in input.chars() {
        match map.entry(nucleotide) {
            Entry::Vacant(_) => return Err(nucleotide),
            Entry::Occupied(mut view) => {
                *view.get_mut() += 1;
            }
        }
    }
    Ok(map)
}
