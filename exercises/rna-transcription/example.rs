/// The possible nucleotides in DNA and RNA
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum Nucleotide {
    Adenine,
    Cytosine,
    Guanine,
    Thymine,
    Uracil,
}

impl Nucleotide {
    /// Parses a nucleotide from its character value, if valid.
    fn from_char(ch: char) -> Option<Nucleotide> {
        Some(match ch {
            'A' => Nucleotide::Adenine,
            'C' => Nucleotide::Cytosine,
            'G' => Nucleotide::Guanine,
            'T' => Nucleotide::Thymine,
            'U' => Nucleotide::Uracil,
            _ => {
                return None;
            }
        })
    }
}

/// A vector of nucleotides
///
/// Guaranteed that Uracil is not present thanks to `new`.
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Dna(Vec<Nucleotide>);

impl Dna {
    /// Parse a DNA string, checking it is valid.
    ///
    /// The error value is the first invalid character index (char index, not utf8).
    pub fn new(input: &str) -> Result<Dna, usize> {
        let mut out = Vec::new();
        for (idx, ch) in input.chars().enumerate() {
            match Nucleotide::from_char(ch) {
                Some(Nucleotide::Uracil) | None => {
                    return Err(idx);
                }
                Some(n) => {
                    out.push(n);
                }
            }
        }
        Ok(Dna(out))
    }

    pub fn into_rna(mut self) -> Rna {
        for nuc in self.0.iter_mut() {
            *nuc = match *nuc {
                Nucleotide::Adenine => Nucleotide::Uracil,
                Nucleotide::Cytosine => Nucleotide::Guanine,
                Nucleotide::Guanine => Nucleotide::Cytosine,
                Nucleotide::Thymine => Nucleotide::Adenine,
                Nucleotide::Uracil => unreachable!(),
            }
        }
        Rna(self.0)
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Rna(Vec<Nucleotide>);

impl Rna {
    /// Parse a RNA string, checking it is valid.
    ///
    /// The error value is the first invalid character index (char index, not utf8).
    pub fn new(input: &str) -> Result<Rna, usize> {
        let mut out = Vec::new();
        for (idx, ch) in input.chars().enumerate() {
            match Nucleotide::from_char(ch) {
                Some(Nucleotide::Thymine) | None => {
                    return Err(idx);
                }
                Some(n) => {
                    out.push(n);
                }
            }
        }
        Ok(Rna(out))
    }
}
