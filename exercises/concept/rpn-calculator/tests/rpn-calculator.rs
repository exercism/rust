use rpn_calculator::*;
use CalculatorInput::*;

#[test]
fn test_empty_input_returns_none() {
    let input = vec![];
    assert_eq!(evaluate(&input), None);
}

#[test]
#[ignore]
fn test_simple_value() {
    let input = vec![Value(10)];
    assert_eq!(evaluate(&input), Some(10));
}

#[test]
#[ignore]
fn test_simple_addition() {
    let input = vec![Value(2), Value(2), Add];
    assert_eq!(evaluate(&input), Some(4));
}

#[test]
#[ignore]
fn test_simple_subtraction() {
    let input = vec![Value(7), Value(11), Subtract];
    assert_eq!(evaluate(&input), Some(-4));
}

#[test]
#[ignore]
fn test_simple_multiplication() {
    let input = vec![Value(6), Value(9), Multiply];
    assert_eq!(evaluate(&input), Some(54));
}

#[test]
#[ignore]
fn test_simple_division() {
    let input = vec![Value(57), Value(19), Divide];
    assert_eq!(evaluate(&input), Some(3));
}

#[test]
#[ignore]
fn test_complex_operation() {
    let input = vec![
        Value(4),
        Value(8),
        Add,
        Value(7),
        Value(5),
        Subtract,
        Divide,
    ];
    assert_eq!(evaluate(&input), Some(6));
}

#[test]
#[ignore]
fn test_too_few_operands_returns_none() {
    let input = vec![Value(2), Add];
    assert_eq!(evaluate(&input), None);
}

#[test]
#[ignore]
fn test_too_many_operands_returns_none() {
    let input = vec![Value(2), Value(2)];
    assert_eq!(evaluate(&input), None);
}
