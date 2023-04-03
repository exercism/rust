pub fn escape_double_quotes(input: &str) -> String {
    let mut output = String::new();
    let mut brace_type = None;

    let mut chars = input.chars().peekable();
    while let Some(char) = chars.next() {
        match char {
            '$' if chars.peek() == Some(&'{') => {
                brace_type = Some("${");
                output.push(char)
            }
            '}' if chars.peek() == Some(&'$') => {
                brace_type = Some("}$");
                output.push(char)
            }
            '"' if brace_type == Some("}$") || brace_type == None => output.push_str("\\\""),
            _ => output.push(char),
        }
    }

    output
}
