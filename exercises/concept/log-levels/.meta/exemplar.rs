pub fn message(log_line: &str) -> String {
    let mut result = String::new();

    let mut found_separator = false;

    for c in log_line.chars() {
        if c == ':' {
            found_separator = true;
            continue;
        }

        if found_separator {
            result.push(c);
        }
    }

    result.trim().to_string()
}

pub fn log_level(log_line: &str) -> String {
    let mut result = String::new();

    for c in log_line.chars() {
        if c == '[' {
            continue;
        }
        if c == ']' {
            break;
        }
        result.push(c);
    }

    result.to_lowercase()
}

pub fn reformat(log_line: &str) -> String {
    let mut result = String::new();
    result.push_str(&message(log_line));
    result.push_str(" (");
    result.push_str(&log_level(log_line));
    result.push(')');

    result
}
