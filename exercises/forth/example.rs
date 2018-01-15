use std::collections::HashMap;
use std::collections::LinkedList;
use std::str::FromStr;

use Term::*;

pub type Value = i32;
pub type ForthResult = Result<(), Error>;
type StackResult<T> = Result<T, Error>;

type Stack = LinkedList<Value>;
type Code = LinkedList<Term>;
type Definitions = HashMap<String, Code>;

pub struct Forth {
    code: Code,
    defs: Definitions,
    stack: Stack,
}

#[derive(Debug, PartialEq)]
pub enum Error {
    DivisionByZero,
    StackUnderflow,
    UnknownWord,
    InvalidWord,
}

#[derive(Debug, Clone)]
enum Term {
    Number(Value),
    Word(String),
    StartDefinition,
    EndDefinition,
}

impl FromStr for Term {
    type Err = ();

    fn from_str(s: &str) -> Result<Term, ()> {
        match s {
            ":" => Ok(StartDefinition),
            ";" => Ok(EndDefinition),
            _   => Err(())
        }
        .or_else(|_| Value::from_str(s).map(Number))
        .or_else(|_| Ok(Word(s.to_ascii_lowercase())))
    }
}

impl Forth {
    pub fn new() -> Forth {
        Forth {
            code: LinkedList::new(),
            defs: HashMap::new(),
            stack: LinkedList::new(),
        }
    }

    pub fn stack(&self) -> Vec<Value> {
        self.stack.iter().cloned().collect()
    }

    pub fn eval(&mut self, input: &str) -> ForthResult {
        let mut new_code = Forth::into_code(input);
        self.code.append(&mut new_code);
        self.run()
    }

    fn run(&mut self) -> ForthResult {
        while let Some(term) = self.code.pop_front() {
            try!(self.step_term(term))
        }

        Forth::ok()
    }

    fn step_term(&mut self, term: Term) -> ForthResult {
        match term {
            Number(value)   => self.push(value),
            Word(word)      => self.step_word(word),
            StartDefinition => self.store_definition(),
            EndDefinition   => Err(Error::InvalidWord),
        }
    }

    fn step_word(&mut self, word: String) -> ForthResult {
        self.defs.get(&word)
            .ok_or(Error::UnknownWord)
            .map(Clone::clone)
            .map(|mut code| self.code.append(&mut code))
            .or_else(|_| self.step_built_in(&word))
    }

    fn step_built_in(&mut self, word: &String) -> ForthResult {
        match word.as_ref() {
            "+" =>
                self.bin_op(|(a, b)| Ok(a + b)),
            "-" =>
                self.bin_op(|(a, b)| Ok(a - b)),
            "*" =>
                self.bin_op(|(a, b)| Ok(a * b)),
            "/" =>
                self.bin_op(|(a, b)| {
                    a.checked_div(b).ok_or(Error::DivisionByZero)
                }),
            "dup" =>
                self.pop().and_then(|a| {
                    self.push(a).and(self.push(a))
                }),
            "drop" =>
                self.pop().and(Forth::ok()),
            "swap" =>
                self.pop_two().and_then(|(a, b)| {
                    self.push(b).and(self.push(a))
                }),
            "over" =>
                self.pop_two().and_then(|(a, b)| {
                    self.push(a).and(self.push(b)).and(self.push(a))
                }),
            _ =>
                Err(Error::UnknownWord)
        }
    }

    fn store_definition(&mut self) -> ForthResult {
        let mut def = LinkedList::new();

        loop {
            match self.code.pop_front() {
                Some(EndDefinition) => break,
                Some(term)          => def.push_back(term),
                None                => return Err(Error::InvalidWord),
            }
        }

        if let Some(Word(name)) = def.pop_front() {
            self.store_word(name, def)
        } else {
            Err(Error::InvalidWord)
        }
    }

    fn push(&mut self, value: Value) -> ForthResult {
        self.stack.push_back(value);
        Forth::ok()
    }

    fn pop(&mut self) -> StackResult<Value> {
        self.stack.pop_back()
            .ok_or(Error::StackUnderflow)
    }

    fn pop_two(&mut self) -> StackResult<(Value, Value)> {
        self.pop()
            .and_then(|b| {
                self.pop()
                    .and_then(|a| Ok((a, b)))
            })
    }

    fn bin_op<F>(&mut self, op: F) -> ForthResult
        where F: FnOnce((Value, Value)) -> StackResult<Value> {
        self.pop_two()
            .and_then(op)
            .and_then(|value| self.push(value))
    }

    fn store_word(&mut self, name: String, code: Code) -> ForthResult {
        self.defs.insert(name, code);
        Forth::ok()
    }

    fn into_code(input: &str) -> LinkedList<Term> {
        input
            .split(|c: char| c.is_whitespace() || c.is_control())
            .map(Term::from_str)
            .filter(Result::is_ok)
            .map(Result::unwrap)
            .collect()
    }

    fn ok() -> ForthResult { Ok(()) }
}
