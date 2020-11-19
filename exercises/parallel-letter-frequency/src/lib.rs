use std::collections::HashMap;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    unimplemented!(
        "Count the frequency of letters in the given input '{:?}'. Ensure that you are using {} to process the input.",
        input,
        match worker_count {
            1 => "1 worker".to_string(),
            _ => format!("{} workers", worker_count),
        }
    );
}
