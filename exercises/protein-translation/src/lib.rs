pub struct CodonsInfo;

impl CodonsInfo {
    pub fn name_for(&self, codon: &str) -> Result<&'static str, &'static str> {
        unimplemented!(
            "Return the protein name for a '{}' codon or an error, if codon string is invalid",
            codon
        );
    }

    pub fn of_rna(&self, rna: &str) -> Result<Vec<&'static str>, &'static str> {
        unimplemented!("Return a list of protein names that correspond to the '{}' RNA string or an error if the RNA string is invalid", rna);
    }
}

pub fn parse(pairs: Vec<(&'static str, &'static str)>) -> CodonsInfo {
    unimplemented!(
        "Construct a new CodonsInfo struct from given pairs: {:?}",
        pairs
    );
}
