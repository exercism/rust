extern crate alphametics;
use std::collections::HashMap;

fn test_helper(puzzle: &str, solution:  &[(char, u8)])  {
    let answer = alphametics::solve(puzzle).unwrap();
    let solution: HashMap<char, u8> = solution.iter().cloned().collect();
    assert_eq!(answer, solution);
}

#[test]
fn test_with_three_letters() {
    test_helper("I + BB == ILL",
      &[('I', 1), ('B', 9), ('L', 0)]);
}

#[test]
#[ignore]
fn test_must_have_unique_value_for_each_letter() {
    let answer = alphametics::solve("A == B");
    assert_eq!(answer, None);
}

#[test]
#[ignore]
fn test_leading_zero_solution_is_invalid() {
    let answer = alphametics::solve("ACA + DD == BD");
    assert_eq!(answer, None);
}

#[test]
#[ignore]
fn test_puzzle_with_four_letters() {
    test_helper("AS + A == MOM", &[('A', 9), ('S', 2), ('M', 1), ('O', 0)]);
}

#[test]
#[ignore]
fn test_puzzle_with_six_letters() {
    test_helper("NO + NO + TOO == LATE",
      &[('N', 7), ('O', 4), ('T', 9), ('L', 1), ('A', 0), ('E', 2)]);
}

#[test]
#[ignore]
fn test_puzzle_with_seven_letters() {
    test_helper("HE + SEES + THE == LIGHT",
      &[('E', 4), ('G', 2), ('H', 5), ('I', 0), ('L', 1), ('S', 9), ('T', 7)]);
}

#[test]
#[ignore]
fn test_puzzle_with_eight_letters() {
    test_helper("SEND + MORE == MONEY",
      &[('S', 9), ('E', 5), ('N', 6), ('D', 7),
        ('M', 1), ('O', 0), ('R', 8), ('Y', 2)]);
}

#[test]
#[ignore]
fn test_puzzle_with_ten_letters() {
    test_helper("AND + A + STRONG + OFFENSE + AS + A + GOOD == DEFENSE",
      &[ ('A', 5), ('D', 3), ('E', 4), ('F', 7), ('G', 8),
         ('N', 0), ('O', 2), ('R', 1), ('S', 6), ('T', 9)]);
}
