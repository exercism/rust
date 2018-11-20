pub fn reverse(input: &str) -> String {
    let mut output = String::with_capacity(input.len());
    output.extend(input.chars().rev());
    output
}
