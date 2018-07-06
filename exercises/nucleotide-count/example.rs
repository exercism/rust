use std::collections::HashMap;

static VALID_NUCLEOTIDES: &'static str = "ACGT";

pub fn count(nucleotide: char, input: &str) -> Result<usize, char> {
    let valid = |x: char| VALID_NUCLEOTIDES.contains(x);

    if !valid(nucleotide) {
        Err(nucleotide)
    } else {
        match input.chars().find(|&c| !valid(c)) {
            Some(c) => Err(c),
            None => Ok(input.chars().filter(|&c| c == nucleotide).count()),
        }
    }
}

pub fn nucleotide_counts(input: &str) -> Result<HashMap<char, usize>, char> {
    let mut map: HashMap<char, usize> = VALID_NUCLEOTIDES.chars().map(|c| (c, 0)).collect();
    for nucleotide in input.chars() {
        if let Some(n) = map.get_mut(&nucleotide) {
            *n += 1;
        } else {
            return Err(nucleotide);
        }
    }
    Ok(map)
}
