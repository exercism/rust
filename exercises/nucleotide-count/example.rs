use std::collections::HashMap;

pub fn count(nucleotide: char, input: &str) -> usize {
    input.chars().filter(|&c| c == nucleotide).count()
}

pub fn nucleotide_counts(input: &str) -> HashMap<char, usize> {
    let mut map: HashMap<char, usize> = "ACGT".chars().map(|c| (c, 0)).collect();
    for nucleotide in input.chars() {
        *map.entry(nucleotide).or_insert(0) += 1;
    }
    map
}
