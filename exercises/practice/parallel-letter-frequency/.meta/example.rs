use std::collections::HashMap;
use std::sync::mpsc::channel;
use std::thread;

/// Compute the frequency of each letter (technically of each unicode codepoint) using the given
/// number of worker threads.
pub fn frequency(texts: &[&str], num_workers: usize) -> HashMap<char, usize> {
    assert!(num_workers > 0);
    let part_size_floor = texts.len() / num_workers;
    let rem = texts.len() % num_workers;
    let part_size = if rem > 0 {
        part_size_floor + 1
    } else {
        part_size_floor
    };
    let mut parts: Vec<Vec<String>> = Vec::with_capacity(part_size);
    for _ in 0..num_workers {
        parts.push(Vec::with_capacity(part_size));
    }
    let mut i = 0;
    for line in texts.iter() {
        // We'll need to clone those strings in order to satisfy some lifetime guarantees. Basically
        // it's hard for the system to be sure that the threads spawned don't outlive the strings.
        parts[i].push(line.to_string());
        i = (i + 1) % num_workers;
    }

    let (tx, rx) = channel();

    for part in parts {
        let tx = tx.clone();
        thread::spawn(move || {
            tx.send(count(&part)).unwrap();
        });
    }

    let mut results: HashMap<char, usize> = HashMap::new();
    for _ in 0..num_workers {
        let part_results = rx.recv().unwrap();
        for (c, n) in part_results.into_iter() {
            *results.entry(c).or_insert(0) += n;
        }
    }
    results
}

fn count(lines: &[String]) -> HashMap<char, usize> {
    let mut results: HashMap<char, usize> = HashMap::new();
    for line in lines.iter() {
        for c in line.chars() {
            if c.is_alphabetic() {
                *results.entry(c.to_lowercase().next().unwrap()).or_insert(0) += 1;
            }
        }
    }
    results
}
