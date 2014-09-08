use std::collections::HashMap;

pub fn count(nucleotide: char, input: &str) -> uint {
    input.chars().filter(|&c| c == nucleotide).count()
}

pub fn nucleotide_counts(input: &str) -> HashMap<char, uint> {
    let mut map: HashMap<char, uint> = "ACGT".chars().map(|c| (c, 0)).collect();
    for nucleotide in input.chars() {
        map.insert_or_update_with(nucleotide, 1, |_, count| *count += 1);
    }
    map
}
