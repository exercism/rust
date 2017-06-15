use std::collections::HashMap;

pub struct CodonInfo<'a> {
    actual_codons: HashMap<&'a str, &'a str>
}

pub fn parse<'a>(pairs: Vec<(&'a str, &'a str)>) -> CodonInfo<'a> {
    CodonInfo{
        actual_codons: pairs.into_iter().collect()
    }
}

impl<'a> CodonInfo<'a> {
    pub fn name_for(&self, codon: &str) -> Result<&'a str, &'static str> {
        match self.actual_codons.get(&codon) {
            Some(name) => Ok(name),
            None => Err("Invalid")
        }
    }
}
