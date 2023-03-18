#[derive(Debug, PartialEq, Eq)]
pub struct Dna;

#[derive(Debug, PartialEq, Eq)]
pub struct Rna;

impl Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        todo!("Construct new Dna from '{dna}' string. If string contains invalid nucleotides return index of first invalid nucleotide");
    }

    pub fn into_rna(self) -> Rna {
        todo!("Transform Dna {self:?} into corresponding Rna");
    }
}

impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        todo!("Construct new Rna from '{rna}' string. If string contains invalid nucleotides return index of first invalid nucleotide");
    }
}
