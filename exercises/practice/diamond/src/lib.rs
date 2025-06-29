use once_cell::sync::Lazy;

static ALPHABET: Lazy<&str> = Lazy::new(|| {
    "ABCDEFGHIJKLMNOPQRSTUVWXYZ"
});

pub fn get_diamond(c: char) -> Vec<String> {

    let mut res: Vec<String> = Vec::new();

    if let Some(c_index) = ALPHABET.find(c) {
        // Special case for A
        res.push(format!("{}A{}", 
                " ".repeat(c_index),
                " ".repeat(c_index)));
        
        // Then, letters up to `c`
        for i in 1..=c_index {
            let letter = ALPHABET.chars().nth(i).unwrap();
            res.push(format!("{}{}{}{}{}", 
                " ".repeat(c_index-i),
                letter,
                " ".repeat(2*i-1),
                letter,
                " ".repeat(c_index-i)));
        }

        // Generate the symmetric bottom half
        let res2: Vec<String> = res[..res.len()-1]
        .iter()
        .rev()
        .cloned()
        .collect(); 

        res.extend(res2);
    }
    res
}
