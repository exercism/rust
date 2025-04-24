use once_cell::sync::Lazy;
use std::collections::HashMap;

static DNA_TO_RNA_MAP: Lazy<HashMap<char, char>> = Lazy::new(|| {
    let mut m: HashMap<char, char> = HashMap::new();
    m.insert('G', 'C');
    m.insert('C', 'G');
    m.insert('T', 'A');
    m.insert('A', 'U');
    m
});

#[derive(Debug, PartialEq, Eq)]
pub struct Dna {
    nucleotides: String
}

#[derive(Debug, PartialEq, Eq)]
pub struct Rna {
    nucleotides : String
}


impl Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        let keys: Vec<char> = DNA_TO_RNA_MAP.keys().cloned().collect();

        match dna.char_indices()
        .find(|(_i, c)| !keys.contains(c))
        .map(|(i, _c)| i) {
            Some(i) => Err(i),
            None => Ok(Self{
                nucleotides: dna.to_string()
            })
        }
    }

    pub fn into_rna(self) -> Rna {
        let rna_nucleotides: String = self.nucleotides
        .chars()
        .map(|c| DNA_TO_RNA_MAP.get(&c).unwrap())
        .collect();

        Rna {
            nucleotides: rna_nucleotides
        }
    }
}

impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        let values: Vec<char> = DNA_TO_RNA_MAP.values().cloned().collect();

        match rna.char_indices()
        .find(|(_i, c)| !values.contains(c))
        .map(|(i, _c)| i) {
            Some(i) => Err(i),
            None => Ok(
                Self{
                    nucleotides: rna.to_string()
                }
            )
        }
    }
}
