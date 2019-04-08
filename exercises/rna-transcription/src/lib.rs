#[derive(Debug, PartialEq)]
pub struct DNA;

#[derive(Debug, PartialEq)]
pub struct RNA;

impl DNA {
    pub fn new(dna: &str) -> Result<DNA, usize> {
        unimplemented!("Construct new DNA from '{}' string. If string contains invalid nucleotides return index of first invalid nucleotide", dna);
    }

    pub fn into_rna(self) -> RNA {
        unimplemented!("Transform DNA {:?} into corresponding RNA", self);
    }
}

impl RNA {
    pub fn new(rna: &str) -> Result<RNA, usize> {
        unimplemented!("Construct new RNA from '{}' string. If string contains invalid nucleotides return index of first invalid nucleotide", rna);
    }
}
