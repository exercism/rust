use std::collections::HashMap;

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {

    if !(nucleotide == 'A' || nucleotide == 'C' || nucleotide == 'G' || nucleotide == 'T') {
        return Err(nucleotide);
    }

    match nucleotide_counts(dna) {
        Ok(res_all) => {
            if let Some(&res) = res_all.get(&nucleotide) {
                Ok(res)
            }
            else {
                Err(nucleotide)
            }
        },
        Err(c) => {
            Err(c)
        }
    }
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {

    let non_nucleotides:String = dna.chars().filter(|&c| !(c == 'A' || c == 'C' || c == 'G' || c == 'T')).collect();

    if !non_nucleotides.is_empty() {
        return Err(non_nucleotides.chars().last().unwrap())
    }

    let mut res: HashMap<char, usize> = HashMap::new();

    for c in ['A', 'C', 'G', 'T'] {
        res.insert(c, 0);
    }

    for c in dna.chars() {
        res.entry(c).and_modify(|counter| *counter += 1);
    }

    Ok(res)
}
