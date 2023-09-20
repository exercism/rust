//! This solution authored by exercism user `2deth`:
//! https://exercism.org/mentor/solutions/b6f9f69b03df4b889c9930960cb4a358?iteration_idx=8

use std::collections::HashMap;

pub type Value = i16;
pub type ForthResult = Result<(), Error>;

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    DivisionByZero,
    StackUnderflow,
    UnknownWord,
    InvalidWord,
}

#[derive(Default)]
pub struct Forth {
    stack: Vec<Value>,
    words: HashMap<String, Id>,
    definitions: Vec<Vec<Op>>,
}

// Instead of `usize`, to reduce `size_of::<Op>`, we use `u16`. This puts a
// maximum limit on the number of custom words this implementation can handle,
// but as a practical matter, it's plenty.
type Id = u16;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Op {
    Add,
    Sub,
    Mul,
    Div,
    Dup,
    Drop,
    Swap,
    Over,
    Push(Value),
    Call(Id),
}

impl Forth {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn stack(&self) -> &[Value] {
        &self.stack
    }

    pub fn eval(&mut self, input: &str) -> ForthResult {
        #[derive(Debug, PartialEq, Eq)]
        enum Mode {
            Execution,
            Word,
            Definition(String, Vec<Op>),
        }
        let mut mode = Mode::Execution;

        for token in input.to_uppercase().split_whitespace() {
            match mode {
                Mode::Execution => {
                    if token == ":" {
                        mode = Mode::Word;
                    } else {
                        eval_op(
                            parse_op(token, &self.words)?,
                            &mut self.stack,
                            &self.definitions,
                        )?;
                    }
                }
                Mode::Word => {
                    if token.parse::<Value>().is_ok() {
                        return Err(Error::InvalidWord);
                    }
                    mode = Mode::Definition(token.into(), Vec::new());
                }
                Mode::Definition(_, ref mut definition) => {
                    if token == ";" {
                        if let Mode::Definition(word, definition) =
                            std::mem::replace(&mut mode, Mode::Execution)
                        {
                            self.definitions.push(definition);
                            self.words.insert(word, self.definitions.len() as Id - 1);
                        } else {
                            unreachable!();
                        }
                    } else {
                        definition.push(parse_op(token, &self.words)?);
                    }
                }
            }
        }

        (mode == Mode::Execution)
            .then_some(())
            .ok_or(Error::InvalidWord)
    }
}

fn parse_op(token: &str, words: &HashMap<String, Id>) -> Result<Op, Error> {
    Ok(if let Some(id) = words.get(token) {
        Op::Call(*id)
    } else {
        match token {
            "+" => Op::Add,
            "-" => Op::Sub,
            "*" => Op::Mul,
            "/" => Op::Div,
            "DUP" => Op::Dup,
            "DROP" => Op::Drop,
            "SWAP" => Op::Swap,
            "OVER" => Op::Over,
            _ => Op::Push(token.parse::<Value>().map_err(|_| Error::UnknownWord)?),
        }
    })
}

fn eval_op(op: Op, stack: &mut Vec<Value>, definitions: &Vec<Vec<Op>>) -> ForthResult {
    let mut pop = || stack.pop().ok_or(Error::StackUnderflow);
    match op {
        Op::Add => {
            let (rhs, lhs) = (pop()?, pop()?);
            stack.push(lhs + rhs);
        }
        Op::Sub => {
            let (rhs, lhs) = (pop()?, pop()?);
            stack.push(lhs - rhs);
        }
        Op::Mul => {
            let (rhs, lhs) = (pop()?, pop()?);
            stack.push(lhs * rhs);
        }
        Op::Div => {
            let (rhs, lhs) = (pop()?, pop()?);
            let quotient = lhs.checked_div(rhs).ok_or(Error::DivisionByZero)?;
            stack.push(quotient);
        }
        Op::Dup => {
            let top = *stack.last().ok_or(Error::StackUnderflow)?;
            stack.push(top);
        }
        Op::Drop => {
            pop()?;
        }
        Op::Swap => {
            let (top, below) = (pop()?, pop()?);
            stack.push(top);
            stack.push(below);
        }
        Op::Over => {
            let below = *stack.iter().nth_back(1).ok_or(Error::StackUnderflow)?;
            stack.push(below);
        }
        Op::Push(num) => {
            stack.push(num);
        }
        Op::Call(fn_id) => {
            for op in &definitions[fn_id as usize] {
                eval_op(*op, stack, definitions)?;
            }
        }
    }
    Ok(())
}
