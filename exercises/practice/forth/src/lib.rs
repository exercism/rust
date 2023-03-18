pub type Value = i32;
pub type Result = std::result::Result<(), Error>;

pub struct Forth;

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    DivisionByZero,
    StackUnderflow,
    UnknownWord,
    InvalidWord,
}

impl Forth {
    pub fn new() -> Forth {
        todo!()
    }

    pub fn stack(&self) -> &[Value] {
        todo!()
    }

    pub fn eval(&mut self, input: &str) -> Result {
        todo!("result of evaluating '{input}'")
    }
}
