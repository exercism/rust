pub type Value = i32;
pub type ForthResult = Result<(), Error>;

pub struct Forth;

#[derive(Debug, PartialEq)]
pub enum Error {
    DivisionByZero,
    StackUnderflow,
    UnknownWord,
    InvalidWord,
}

impl Forth {
    pub fn new() -> Forth {
    }

    pub fn stack(&self) -> Vec<Value> {
    }

    pub fn eval(&mut self, input: &str) -> ForthResult {
    }
}
