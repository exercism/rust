#[derive(PartialEq, Eq, Debug)]
pub struct RibonucleicAcid {
    nucleotides: String
}

impl RibonucleicAcid {
    pub fn new(nucleotides: &str) -> RibonucleicAcid {
        RibonucleicAcid { nucleotides: nucleotides.to_string() }
    }
}

#[derive(PartialEq, Eq, Debug)]
pub struct DeoxyribonucleicAcid {
    nucleotides: String
}

fn transcribe_dna_rna(c: char) -> Option<char> {
    match c {
        'C' => Some('G'),
        'G' => Some('C'),
        'A' => Some('U'),
        'T' => Some('A'),
        _   => None
    }
}

impl DeoxyribonucleicAcid {
    pub fn new(nucleotides: &str) -> DeoxyribonucleicAcid {
        DeoxyribonucleicAcid { nucleotides: nucleotides.to_string() }
    }

    pub fn to_rna(&self) -> Result<RibonucleicAcid, ()> {
        let rna_nucleotides: String = self.nucleotides.chars().filter_map(transcribe_dna_rna).collect();
        if rna_nucleotides.len() == self.nucleotides.len() {
            Ok(RibonucleicAcid { nucleotides: rna_nucleotides })
        } else {
            Err(())
        }
    }
}
