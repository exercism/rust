pub struct CodonsInfo<'a> {
    // We fake using 'a here, so the compiler does not complain that
    // "parameter `'a` is never used". Delete when no longer needed.
    phantom: std::marker::PhantomData<&'a ()>,
}

impl<'a> CodonsInfo<'a> {
    pub fn name_for(&self, codon: &str) -> Option<&'a str> {
        todo!("Return the protein name for a '{codon}' codon or None, if codon string is invalid");
    }

    pub fn of_rna(&self, rna: &str) -> Option<Vec<&'a str>> {
        todo!("Return a list of protein names that correspond to the '{rna}' RNA string or None if the RNA string is invalid");
    }
}

pub fn parse<'a>(pairs: Vec<(&'a str, &'a str)>) -> CodonsInfo<'a> {
    todo!("Construct a new CodonsInfo struct from given pairs: {pairs:?}");
}
