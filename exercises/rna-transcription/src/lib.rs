#[derive(Debug, PartialEq)]
pub struct Dna;

#[derive(Debug, PartialEq)]
pub struct Rna;

impl Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        unimplemented!("Construct new Dna from '{}' string. If string contains invalid nucleotides return index of first invalid nucleotide", dna);
    }

    pub fn into_rna(self) -> Rna {
        unimplemented!("Transform Dna {:?} into corresponding Rna", self);
    }
}

impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        unimplemented!("Construct new Rna from '{}' string. If string contains invalid nucleotides return index of first invalid nucleotide", rna);
    }
}
