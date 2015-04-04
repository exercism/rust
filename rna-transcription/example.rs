use std::convert::AsRef;

#[derive(PartialEq, Eq, Debug)]
pub struct RibonucleicAcid {
    nucleotides: String
}

impl RibonucleicAcid {
    pub fn new(nucleotides: &str) -> RibonucleicAcid {
        RibonucleicAcid { nucleotides: nucleotides.to_string() }
    }
}

impl AsRef<str> for RibonucleicAcid {
    fn as_ref(&self) -> &str {
        self.nucleotides.as_ref()
    }
}

#[derive(PartialEq, Eq, Debug)]
pub struct DeoxyribonucleicAcid {
    nucleotides: String
}

fn transcribe_dna_rna(c: char) -> char {
    match c {
        'C' => 'G',
        'G' => 'C',
        'A' => 'U',
        'T' => 'A',
        _   => c
    }
}

impl DeoxyribonucleicAcid {
    pub fn new(nucleotides: &str) -> DeoxyribonucleicAcid {
        DeoxyribonucleicAcid { nucleotides: nucleotides.to_string() }
    }

    pub fn to_rna(&self) -> RibonucleicAcid {
        let rna_nucleotides = self.nucleotides.chars().map(transcribe_dna_rna).collect();
        RibonucleicAcid { nucleotides: rna_nucleotides }
    }
}
