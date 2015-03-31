fn sort(word: &String) -> String {
    let mut sorted: Vec<char> = word.chars().collect();
    sorted.sort();
    let mut output = String::with_capacity(sorted.len());
    for c in sorted.drain() {
        output.push(c);
    }
    output
}

pub fn anagrams_for<'a>(word: &str, inputs: &[&'a str]) -> Vec<&'a str> {
    let lower = word.to_lowercase();
    let sorted = sort(&lower);
    let mut anagrams = Vec::new();
    for input in inputs.iter() {
        let input_lower = input.to_lowercase();
        if lower != input_lower {
            if sorted == sort(&input_lower) {
                anagrams.push(*input);
            }
        }
    }
    anagrams
}
