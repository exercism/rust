pub fn escape_double_quotes(input: &str) -> String {
    let mut output = String::new();
    let mut escape = true;

    let mut chars = input.chars().peekable();
    while let Some(char) = chars.next() {
        match char {
            '$' if chars.peek() == Some(&'{') => {
                escape = false;
                output.push(char)
            }
            '}' if chars.peek() == Some(&'$') => {
                escape = true;
                output.push(char)
            }
            '"' if escape => output.push_str("\\\""),
            _ => output.push(char),
        }
    }

    output
}
