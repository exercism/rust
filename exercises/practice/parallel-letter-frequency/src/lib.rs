use std::collections::HashMap;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    todo!(
        "Count the frequency of letters in the given input '{input:?}'. Ensure that you are using {} to process the input.",
        match worker_count {
            1 => "1 worker".to_string(),
            _ => format!("{worker_count} workers"),
        }
    );
}
