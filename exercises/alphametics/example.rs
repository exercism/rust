// This is a brute-force solution, use `cargo test --release` for faster testing

extern crate itertools;
extern crate permutohedron;

use itertools::Itertools;
use permutohedron::Heap as Permutations;

use std::char;
use std::collections::HashMap;
use std::collections::HashSet;

fn test_equation(puzzle: &str, substitutions: &HashMap<char, u8>) -> bool {
    // Create a new String with characters changed to numbers
    let puzzle: String = puzzle
        .chars()
        .map(|c| {
            if let Some(&n) = substitutions.get(&c) {
                // If the character is in the substitutions, get the number and
                // convert it to a char
                char::from_digit(u32::from(n), 10).unwrap()
            } else {
                // Otherwise just copy over the character
                c
            }
        })
        .collect();

    // Split the puzzle into left and right side
    let equation: Vec<&str> = puzzle.split("==").collect();

    // Parse the number on the right side
    let right = equation[1].trim().parse::<u32>().unwrap();

    // Sum the parts on the left side
    let left: u32 = equation[0]
        .split('+')
        .map(str::trim)
        .map(|n| n.parse::<u32>().unwrap())
        .sum();

    // Create a String with just the numbers and spaces
    let just_numbers = puzzle
        .chars()
        .filter(|c| c.is_digit(10) || c.is_whitespace())
        .collect::<String>();
    // Split this into the numbers and check every number's first character
    let no_leading_zeroes = just_numbers
        .split_whitespace()
        .all(|number| !number.starts_with('0'));

    // Return true if left and right side is equal and the equation doesnt
    // contain leading zeroes.
    left == right && no_leading_zeroes
}

pub fn solve(puzzle: &str) -> Option<HashMap<char, u8>> {
    // Get unique letters from the puzzle
    let letters: HashSet<char> = puzzle
        .chars()
        .filter(|&c| c.is_alphabetic() && c.is_uppercase())
        .collect();
    let letters: Vec<char> = letters.into_iter().collect();

    // All available numbers for substitution
    let numbers: &[u8] = &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

    // Iterate every combination with the length of unique letters in the puzzle
    for combinations in numbers.iter().combinations(letters.len()) {
        let mut c = combinations;
        let permutations = Permutations::new(&mut c);
        // Iterate every permutation of a letter combination
        for p in permutations {
            let substitution: HashMap<char, u8> =
                letters.iter().zip(p).map(|(&c, &n)| (c, n)).collect();
            if test_equation(puzzle, &substitution) {
                // We found a good substitution
                return Some(substitution);
            }
        }
    }
    // If we tested every combination and did not found a solution then return None
    None
}
