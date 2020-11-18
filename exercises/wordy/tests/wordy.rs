use wordy::answer;

#[test]
fn just_a_number() {
    let command = "What is 5?";
    assert_eq!(Some(5), answer(command));
}

#[test]
#[ignore]
fn addition() {
    let command = "What is 1 plus 1?";
    assert_eq!(Some(2), answer(command));
}

#[test]
#[ignore]
fn more_addition() {
    let command = "What is 53 plus 2?";
    assert_eq!(Some(55), answer(command));
}

#[test]
#[ignore]
fn addition_with_negative_numbers() {
    let command = "What is -1 plus -10?";
    assert_eq!(Some(-11), answer(command));
}

#[test]
#[ignore]
fn large_addition() {
    let command = "What is 123 plus 45678?";
    assert_eq!(Some(45_801), answer(command));
}

#[test]
#[ignore]
fn subtraction() {
    let command = "What is 4 minus -12?";
    assert_eq!(Some(16), answer(command));
}

#[test]
#[ignore]
fn multiplication() {
    let command = "What is -3 multiplied by 25?";
    assert_eq!(Some(-75), answer(command));
}

#[test]
#[ignore]
fn division() {
    let command = "What is 33 divided by -3?";
    assert_eq!(Some(-11), answer(command));
}

#[test]
#[ignore]
fn multiple_additions() {
    let command = "What is 1 plus 1 plus 1?";
    assert_eq!(Some(3), answer(command));
}

#[test]
#[ignore]
fn addition_and_subtraction() {
    let command = "What is 1 plus 5 minus -2?";
    assert_eq!(Some(8), answer(command));
}

#[test]
#[ignore]
fn multiple_subtraction() {
    let command = "What is 20 minus 4 minus 13?";
    assert_eq!(Some(3), answer(command));
}

#[test]
#[ignore]
fn subtraction_then_addition() {
    let command = "What is 17 minus 6 plus 3?";
    assert_eq!(Some(14), answer(command));
}

#[test]
#[ignore]
fn multiple_multiplications() {
    let command = "What is 2 multiplied by -2 multiplied by 3?";
    assert_eq!(Some(-12), answer(command));
}

#[test]
#[ignore]
fn addition_and_multiplication() {
    let command = "What is -3 plus 7 multiplied by -2?";
    assert_eq!(Some(-8), answer(command));
}

#[test]
#[ignore]
fn multiple_divisions() {
    let command = "What is -12 divided by 2 divided by -3?";
    assert_eq!(Some(2), answer(command));
}

#[test]
#[ignore]
fn unknown_operation() {
    let command = "What is 52 cubed?";
    assert_eq!(None, answer(command));
}

#[test]
#[ignore]
fn non_math_question() {
    let command = "Who is the President of the United States?";
    assert_eq!(None, answer(command));
}

#[test]
#[ignore]
fn reject_problem_missing_an_operand() {
    let command = "What is 1 plus?";
    assert_eq!(None, answer(command));
}

#[test]
#[ignore]
fn reject_problem_with_no_operands_or_operators() {
    let command = "What is?";
    assert_eq!(None, answer(command));
}

#[test]
#[ignore]
fn reject_two_operations_in_a_row() {
    let command = "What is 1 plus plus 2?";
    assert_eq!(None, answer(command));
}

#[test]
#[ignore]
fn reject_two_numbers_in_a_row() {
    let command = "What is 1 plus 2 1?";
    assert_eq!(None, answer(command));
}

#[test]
#[ignore]
fn reject_postfix_notation() {
    let command = "What is 1 2 plus?";
    assert_eq!(None, answer(command));
}

#[test]
#[ignore]
fn reject_prefix_notation() {
    let command = "What is plus 1 2?";
    assert_eq!(None, answer(command));
}

#[test]
#[ignore]
#[cfg(feature = "exponentials")]
fn exponential() {
    let command = "What is 2 raised to the 5th power?";
    assert_eq!(Some(32), answer(command));
}

#[test]
#[ignore]
#[cfg(feature = "exponentials")]
fn addition_and_exponential() {
    let command = "What is 1 plus 2 raised to the 2nd power?";
    assert_eq!(Some(9), answer(command));
}
