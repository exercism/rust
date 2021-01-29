// This is a brute-force solution, use `cargo test --release` for faster testing

extern crate itertools;
extern crate permutohedron;

use itertools::Itertools;
use permutohedron::Heap as Permutations;

use std::char;
use std::collections::HashMap;
use std::collections::HashSet;

fn test_equation(
    coefficients: &HashMap<char, i64>,
    cant_be_zero: &HashSet<char>,
    substitutions: &HashMap<char, u8>,
) -> bool {
    if cant_be_zero.iter().any(|d| substitutions[d] == 0) {
        return false;
    }

    coefficients
        .iter()
        .map(|(d, &coeff)| i64::from(substitutions[d]) * coeff)
        .sum::<i64>()
        == 0
}

fn letter_coefficients(puzzle: &str) -> (HashMap<char, i64>, HashSet<char>) {
    let mut coefficients = HashMap::new();
    let mut cant_be_zero = HashSet::new();

    // Split the puzzle into left and right side
    let equation: Vec<&str> = puzzle.split("==").collect();

    {
        let mut insert_term = |term: &str, polarity: i64| {
            cant_be_zero.insert(term.chars().next().unwrap());
            let mut power_of_ten = polarity;
            for letter in term.chars().rev() {
                *coefficients.entry(letter).or_insert(0) += power_of_ten;
                power_of_ten *= 10;
            }
        };

        // Sum the parts on the left side
        for left_term in equation[0].split('+').map(str::trim) {
            insert_term(left_term, 1);
        }

        let right_term = equation[1].trim();
        insert_term(right_term, -1);
    }

    (coefficients, cant_be_zero)
}

pub fn solve(puzzle: &str) -> Option<HashMap<char, u8>> {
    let (coefficients, cant_be_zero) = letter_coefficients(puzzle);
    let letters: Vec<char> = coefficients.keys().cloned().collect();

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
            if test_equation(&coefficients, &cant_be_zero, &substitution) {
                // We found a good substitution
                return Some(substitution);
            }
        }
    }
    // If we tested every combination and did not found a solution then return None
    None
}
