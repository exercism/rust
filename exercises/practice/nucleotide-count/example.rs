use std::collections::HashMap;

static VALID_NUCLEOTIDES: &str = "ACGT";

fn valid(c: char) -> Result<char, char> {
    if VALID_NUCLEOTIDES.contains(c) {
        Ok(c)
    } else {
        Err(c)
    }
}

pub fn count(nucleotide: char, input: &str) -> Result<usize, char> {
    valid(nucleotide)?;
    let mut count = 0;
    for c in input.chars() {
        if valid(c)? == nucleotide {
            count += 1;
        }
    }
    Ok(count)
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
