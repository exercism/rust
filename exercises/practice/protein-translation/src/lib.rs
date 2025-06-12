pub fn translate(rna: &str) -> Option<Vec<&str>> {
    // Let's suppose the string is ASCII-only

    let bytes_len = rna.bytes().len();

    let codons = (0..bytes_len)
    .step_by(3)
    .map(|i| &rna[i..(i+3).min(bytes_len)]);

    let mut res: Vec<&str> = Vec::new();

    for codon in codons {
        match codon {
            "AUG" => res.push("Methionine"),
            "UUU" | "UUC" => res.push("Phenylalanine"),
            "UUA" | "UUG" => res.push("Leucine"),
            "UCU" | "UCC" | "UCA" | "UCG" => res.push("Serine"),
            "UAU" | "UAC" => res.push("Tyrosine"),
            "UGU" | "UGC" => res.push("Cysteine"),
            "UGG" => res.push("Tryptophan"),
            "UAA" | "UAG" | "UGA" => {
                break;
            },
            _ => {
                return None;
            }
        }
    }

    Some(res)
}
