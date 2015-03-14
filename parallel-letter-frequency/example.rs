use std::ascii::AsciiExt;
use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::sync::Future;

/// Compute the frequency of each letter (technically of each unicode codepoint) using the given
/// number of worker threads.
pub fn frequency(texts: &[&str], num_workers: usize) -> HashMap<char, usize> {
    assert!(num_workers > 0);
    let part_size_floor = texts.len() / num_workers;
    let rem = texts.len() % num_workers;
    let part_size = if rem > 0 { part_size_floor + 1 } else { part_size_floor };
    let mut parts: Vec<Vec<String>> = Vec::with_capacity(part_size);
    for _ in 0 .. num_workers {
        parts.push(Vec::with_capacity(part_size));
    }
    let mut i = 0;
    for line in texts.iter() {
        // We'll need to clone those strings in order to satisfy some lifetime guarantees. Basically
        // it's hard for the system to be sure that the threads spawned don't outlive the strings.
        parts[i].push(line.to_string());
        i = (i + 1) % num_workers;
    }
    let futures = parts.into_iter().map(|part| Future::spawn(move || { count(part) }));
    let mut results: HashMap<char, usize> = HashMap::new();
    for mut fut in futures {
        let part_results = fut.get();
        for (c, n) in part_results.into_iter() {
            match results.entry(c) {
                Entry::Vacant(view) => { view.insert(n); },
                Entry::Occupied(mut view) => {
                    *view.get_mut() += n;
                }
            }
        }
    }
    results
}

fn count(lines: Vec<String>) -> HashMap<char, usize> {
    let mut results: HashMap<char, usize> = HashMap::new();
    for line in lines.iter() {
        for c in line.as_slice().chars() {
            if c.is_alphabetic() {
                match results.entry(c.to_ascii_lowercase()) {
                    Entry::Vacant(view) => { view.insert(1); },
                    Entry::Occupied(mut view) => {
                        *view.get_mut() += 1;
                    }
                }
            }
        }
    }
    results
}
