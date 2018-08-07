use std::collections::HashMap;

pub fn frequency(input: &[&str], worker_count: i32) -> HashMap<char, u32> {
    unimplemented!(
        "Count the frequency of letters in the given input '{:?}'. Ensure that you are using {} to process the input.",
        input,
        match worker_count {
            1 => format!("1 worker"),
            _ => format!("{} workers", worker_count),
        }
    );
}
