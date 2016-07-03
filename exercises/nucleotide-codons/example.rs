use std::collections::HashMap;

pub struct CodonInfo<'a> {
    actual_codons: HashMap<&'a str, &'a str>,
}

pub fn parse<'a>(pairs: Vec<(&'a str, &'a str)>) -> CodonInfo<'a> {
    let mut info = CodonInfo {
        // Allocate once for the worst case, shrink later.
        actual_codons: HashMap::with_capacity(pairs.len()),
    };
    for (codon, name) in pairs.into_iter() {
        info.actual_codons.insert(codon, name);
    }
    info.actual_codons.shrink_to_fit();
    info
}

impl<'a> CodonInfo<'a> {
    pub fn name_for(&self, codon: &str) -> Result<&'a str, &'static str> {
        if codon.len() != 3 {
            return Err("invalid length");
        }

        let mut valid = true;
        let lookup: String =
            codon.chars()
                 .map(|l| {
                     // Get an example of a "letter" represented by the possibly encoded letter.
                     // Since every codon represented by the compressed notation has to be of
                     // the desired amino acid just picking one at random will do.
                     match l {
                         'A' | 'W' | 'M' | 'R' | 'D' | 'H' | 'V' | 'N' => 'A',
                         'C' | 'S' | 'Y' | 'B' => 'C',
                         'G' | 'K' => 'G',
                         'T' => 'T',
                         _ => {
                             valid = false;
                             ' '
                         }
                     }
                 })
                 .collect();
        if !valid {
            return Err("invalid char");
        }

        // If the input table is correct (which it is) every valid codon is in it
        // so unwrap() shouldn't panic.
        Ok(self.actual_codons.get(&lookup.as_ref()).unwrap())
    }
}
