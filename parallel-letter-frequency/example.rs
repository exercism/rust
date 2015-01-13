use std::collections::HashMap;
use std::collections::hash_map::Entry;

/// Compute the frequency of each letter (technically of each unicode codepoint) using the given
/// number of worker threads.
pub fn frequency(texts: &[&str], num_workers: usize) -> HashMap<char, usize> {
    assert!(num_workers > 0);
    let part_size_floor = texts.len() / num_workers;
    let rem = texts.len() % num_workers;
    let part_size = if rem > 0 { part_size_floor + 1 } else { part_size_floor };
    let mut parts: Vec<Vec<String>> = Vec::with_capacity(part_size);
    for _ in range(0, num_workers) {
        parts.push(Vec::with_capacity(part_size));
    }
    let mut i = 0;
    for line in texts.iter() {
        // We'll need to clone those strings in order to satisfy some lifetime guarantees. Basically
        // it's hard for the system to be sure that the threads spawned don't outlive the srings.
        parts.get_mut(i).push(line.into_string());
        i = (i + 1) % num_workers;
    }
    let (tx, rx) = channel();
    for part in parts.into_iter() {
        let tx = tx.clone();
        spawn(move || {
            let part_results = count(part);
            tx.send(part_results);
        });
    }
    let mut results: HashMap<char, usize> = HashMap::new();
    for _ in range(0, num_workers) {
        let part_results = rx.recv();
        for (c, n) in part_results.into_iter() {
            match results.entry(c) {
                Vacant(view) => { view.set(n); },
                Occupied(mut view) => {
                    *view.get_mut() += n;
                    view.into_mut();
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
                match results.entry(c.to_lowercase()) {
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
