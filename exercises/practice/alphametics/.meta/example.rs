use std::collections::HashMap;
use std::iter;

use itertools::Itertools;

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let (terms, solution) = input.split_once(" == ").unwrap();
    let numbers = iter::once(solution)
        .chain(terms.split(" + "))
        .map(|word| word.chars().map(|c| (c as u8) - b'A').collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut unique = numbers.iter().flatten().copied().collect::<Vec<_>>();
    unique.sort_unstable();
    unique.dedup();
    let mut assignment = [0; 26];

    for perm in (0..10).permutations(unique.len()) {
        for (&letter, value) in unique.iter().zip(perm) {
            assignment[letter as usize] = value;
        }

        if numbers.iter().any(|n| assignment[n[0] as usize] == 0) {
            continue; // leading zero is invalid
        }

        let sum: u64 = numbers[1..].iter().map(|n| parse(n, &assignment)).sum();
        let solution = parse(&numbers[0], &assignment);

        if sum == solution {
            return Some(
                unique
                    .into_iter()
                    .map(|c| ((c + b'A') as char, assignment[c as usize]))
                    .collect(),
            );
        }
    }
    None
}

fn parse(number: &[u8], assignment: &[u8; 26]) -> u64 {
    number.iter().fold(0, |acc, next| {
        acc * 10 + (assignment[*next as usize] as u64)
    })
}
