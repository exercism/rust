pub fn encrypt(input: &str) -> String {

    let normalized: Vec<char> = input
    .chars()
    .filter(|c| {
        c.is_alphanumeric()
    })
    .map(|c| {
        c.to_ascii_lowercase()
    })
    .collect();
    
    // Early return
    if normalized.is_empty() {
        return "".to_string();
    }

    let nb_normalized_chars = normalized.len();
    let nb_row = (nb_normalized_chars as f64).sqrt() as usize;
    let nb_col = if nb_row * nb_row < nb_normalized_chars {
        nb_row + 1
    }
    else {
        nb_row
    };
    let nb_row = if nb_row * nb_col < nb_normalized_chars {
        nb_row + 1
    }
    else {
        nb_row
    };

    println!("nb_row: {}, nb_col: {}", nb_row, nb_col);

    let intermediate_representation: Vec<Vec<char>> = normalized
    .chunks(nb_col)
    .map(|chunk| {
        let mut s: String = chunk.iter().collect();
        while s.chars().count() < nb_col {
            s.push(' ');
        }
        s
    })
    .map(|s| {
        s.chars().collect()
    })
    .collect();

    let encoded: Vec<String> = (0..nb_col).map(|col| {
        (0..nb_row).map(|row| {
            intermediate_representation[row][col]
        })
        .collect()
    })
    .collect();
    
    encoded.join(" ")
}
