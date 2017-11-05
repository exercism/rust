extern crate wordy;

use wordy::*;

#[test]
fn addition() {
    let command = "What is 1 plus 1?";
    assert_eq!(Ok(2), WordProblem::new(command).answer());
}

#[test]
#[ignore]
fn more_addition() {
    let command = "What is 53 plus 2?";
    assert_eq!(Ok(55), WordProblem::new(command).answer());
}

#[test]
#[ignore]
fn addition_with_negative_numbers() {
    let command = "What is -1 plus -10?";
    assert_eq!(Ok(-11), WordProblem::new(command).answer());
}

#[test]
#[ignore]
fn large_addition() {
    let command = "What is 123 plus 45678?";
    assert_eq!(Ok(45801), WordProblem::new(command).answer());
}

#[test]
#[ignore]
fn subtraction() {
    let command = "What is 4 minus -12?";
    assert_eq!(Ok(16), WordProblem::new(command).answer());
}

#[test]
#[ignore]
fn multiplication() {
    let command = "What is -3 multiplied by 25?";
    assert_eq!(Ok(-75), WordProblem::new(command).answer());
}

#[test]
#[ignore]
fn division() {
    let command = "What is 33 divided by -3?";
    assert_eq!(Ok(-11), WordProblem::new(command).answer());
}

#[test]
#[ignore]
fn multiple_additions() {
    let command = "What is 1 plus 1 plus 1?";
    assert_eq!(Ok(3), WordProblem::new(command).answer());
}

#[test]
#[ignore]
fn addition_and_subtraction() {
    let command = "What is 1 plus 5 minus -2?";
    assert_eq!(Ok(8), WordProblem::new(command).answer());
}

#[test]
#[ignore]
fn multiple_subtraction() {
    let command = "What is 20 minus 4 minus 13?";
    assert_eq!(Ok(3), WordProblem::new(command).answer());
}

#[test]
#[ignore]
fn subtraction_then_addition() {
    let command = "What is 17 minus 6 plus 3?";
    assert_eq!(Ok(14), WordProblem::new(command).answer());
}

#[test]
#[ignore]
fn multiple_multiplications() {
    let command = "What is 2 multiplied by -2 multiplied by 3?";
    assert_eq!(Ok(-12), WordProblem::new(command).answer());
}

#[test]
#[ignore]
fn addition_and_multiplication() {
    let command = "What is -3 plus 7 multiplied by -2?";
    assert_eq!(Ok(-8), WordProblem::new(command).answer());
}

#[test]
#[ignore]
fn multiple_divisions() {
    let command = "What is -12 divided by 2 divided by -3?";
    assert_eq!(Ok(2), WordProblem::new(command).answer());
}

#[test]
#[ignore]
fn unknown_operation() {
    let command = "What is 52 cubed?";
    assert!(WordProblem::new(command).answer().is_err());
}

#[test]
#[ignore]
fn non_math_question() {
    let command = "Who is the President of the United States?";
    assert!(WordProblem::new(command).answer().is_err());
}
