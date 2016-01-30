fn sort(word: &String) -> String {
    let mut sorted: Vec<char> = word.chars().collect();
    sorted.sort();
    let mut output = String::with_capacity(sorted.len());
    for c in sorted.iter() {
        output.push(*c);
    }
    output
}

fn lowercase(word: &str) -> String {
    let mut lower = String::with_capacity(word.len());
    for c in word.chars() {
        for low in c.to_lowercase() {
            lower.push(low);
        }
    }
    lower
}

pub fn anagrams_for<'a>(word: &str, inputs: &[&'a str]) -> Vec<&'a str> {
    let lower = lowercase(word);
    let sorted = sort(&lower);
    let mut anagrams = Vec::new();
    for input in inputs.iter() {
        let input_lower = lowercase(input);
        if lower != input_lower {
            if sorted == sort(&input_lower) {
                anagrams.push(*input);
            }
        }
    }
    anagrams
}
