pub fn translate(rna: &str) -> Option<Vec<&str>> {
    rna.as_bytes()
        .chunks(3)
        .map(|codon| match codon {
            b"AUG" => Some("methionine"),
            b"UUU" | b"UUC" => Some("phenylalanine"),
            b"UUA" | b"UUG" => Some("leucine"),
            b"UCU" | b"UCC" | b"UCA" | b"UCG" => Some("serine"),
            b"UAU" | b"UAC" => Some("tyrosine"),
            b"UGU" | b"UGC" => Some("cysteine"),
            b"UGG" => Some("tryptophan"),
            b"UAA" | b"UAG" | b"UGA" => Some("STOP"),
            _ => None,
        })
        .take_while(|result| result.is_none() || result.unwrap() != "STOP")
        .collect()
}
