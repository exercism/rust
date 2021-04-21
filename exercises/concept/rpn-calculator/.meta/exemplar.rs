#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let mut stack = Vec::new();

    for input in inputs {
        use CalculatorInput::*;

        match input {
            Add => {
                let rhs = stack.pop()?;
                let lhs = stack.pop()?;
                stack.push(lhs + rhs);
            }
            Subtract => {
                let rhs = stack.pop()?;
                let lhs = stack.pop()?;
                stack.push(lhs - rhs);
            }
            Multiply => {
                let rhs = stack.pop()?;
                let lhs = stack.pop()?;
                stack.push(lhs * rhs);
            }
            Divide => {
                let rhs = stack.pop()?;
                let lhs = stack.pop()?;
                stack.push(lhs / rhs);
            }
            Value(value) => stack.push(*value),
        }
    }

    let output = stack.pop();
    if stack.is_empty() {
        output
    } else {
        None
    }
}
