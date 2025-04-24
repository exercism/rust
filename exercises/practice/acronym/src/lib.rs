fn acronym_from_camelcase(word: &str) -> String {
    let mut res = String::new();
    res.push(word.chars().next().unwrap().to_uppercase().next().unwrap());

    println!("res: {}", res);

    let remaining_capitals = word[1..].chars()
    .filter(|c| c.is_uppercase())
    .collect::<String>();

    println!("remaining_capitals: {}", remaining_capitals);

    if remaining_capitals.chars().count() < word.chars().count() - 1 {
        res.push_str(&remaining_capitals);
    }
    

    println!("res before return: {}", res);

    res
    
}

pub fn abbreviate(phrase: &str) -> String {

    let cleaned_phrase: String = phrase.chars()
    .filter(|&c| c.is_alphanumeric() || c == '-' || c.is_whitespace())
    .map(|c| if c == '-' {' '} else {c})
    .collect();

    println!("{}", cleaned_phrase);
    
    cleaned_phrase.split_whitespace()
    // .map(|word| word.chars().next().unwrap().to_uppercase().to_string())
    .map(acronym_from_camelcase)
    .collect::<String>()

    // "PNG".to_string()
}
