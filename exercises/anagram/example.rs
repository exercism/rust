fn sort(word: &String) -> String {
    let mut sorted: Vec<char> = word.chars().collect();
    sorted.sort();
    let mut output = String::with_capacity(sorted.len());
    for c in sorted.iter() {
        output.push(*c);
    }
    output
}

pub fn anagrams_for<'a>(word: &str, inputs: &[&'a str]) -> Vec<&'a str> {
    let lower = word.to_lowercase();
    let sorted = sort(&lower);
    inputs.iter().filter(|input| {
        let input_lower = input.to_lowercase();
        lower != input_lower && sorted == sort(&input_lower)
    }).cloned().collect()
}
