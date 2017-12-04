#[derive(PartialEq, Eq, Debug)]
pub struct RNA {
    nucleotides: String
}

impl RNA {
    pub fn new(nucleotides: &str) -> RNA {
        RNA { nucleotides: nucleotides.to_string() }
    }
}

#[derive(PartialEq, Eq, Debug)]
pub struct DNA {
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

impl DNA {
    pub fn new(nucleotides: &str) -> DNA {
        DNA { nucleotides: nucleotides.to_string() }
    }

    pub fn to_rna(&self) -> Result<RNA, ()> {
        let rna_nucleotides: String = self.nucleotides.chars().filter_map(transcribe_dna_rna).collect();
        if rna_nucleotides.len() == self.nucleotides.len() {
            Ok(RNA { nucleotides: rna_nucleotides })
        } else {
            Err(())
        }
    }
}
