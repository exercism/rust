#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    todo!(
        "Given the inputs: {inputs:?}, evaluate them as though they were a Reverse Polish notation expression"
    );
}
