use std::collections::HashMap;
use std::collections::hashmap::{Occupied, Vacant};
use std::num::div_rem;

/// Compute the frequency of each letter (technically of each unicode codepoint) using the given
/// number of worker threads.
pub fn frequency(texts: &[&str], num_workers: uint) -> HashMap<char, uint> {
    assert!(num_workers > 0);
    let (part_size_floor, rem) = div_rem(texts.len(), num_workers);
    let part_size = if rem > 0 { part_size_floor + 1 } else { part_size_floor };
    let mut parts: Vec<Vec<String>> = Vec::from_fn(num_workers, |_| Vec::with_capacity(part_size));
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
        spawn(proc() {
            let part_results = count(part);
            tx.send(part_results);
        });
    }
    let mut results: HashMap<char, uint> = HashMap::new();
    for _ in range(0, num_workers) {
        let part_results = rx.recv();
        for (c, n) in part_results.into_iter() {
            match results.entry(c) {
                Vacant(entry) => { entry.set(n); },
                Occupied(mut entry) => {
                    *entry.get_mut() += n;
                    entry.into_mut();
                }
            }
        }
    }
    results
}

fn count(lines: Vec<String>) -> HashMap<char, uint> {
    let mut results: HashMap<char, uint> = HashMap::new();
    for line in lines.iter() {
        for c in line.as_slice().chars() {
            if c.is_alphabetic() {
                match results.entry(c.to_lowercase()) {
                    Vacant(entry) => { entry.set(1); },
                    Occupied(mut entry) => {
                        *entry.get_mut() += 1;
                    }
                }
            }
        }
    }
    results
}
