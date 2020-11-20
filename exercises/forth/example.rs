use std::collections::HashMap;
use std::collections::VecDeque;
use std::str::FromStr;

pub type Value = i32;
pub type ForthResult = Result<(), Error>;
type StackResult<T> = Result<T, Error>;

type Stack = Vec<Value>;
type Code = VecDeque<Term>;
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
            ":" => Ok(Term::StartDefinition),
            ";" => Ok(Term::EndDefinition),
            _ => Err(()),
        }
        .or_else(|_| Value::from_str(s).map(Term::Number))
        .or_else(|_| Ok(Term::Word(s.to_ascii_lowercase())))
    }
}

impl Forth {
    pub fn new() -> Forth {
        Forth {
            code: Code::new(),
            defs: HashMap::new(),
            stack: Stack::new(),
        }
    }

    pub fn stack(&self) -> &[Value] {
        self.stack.as_slice()
    }

    pub fn eval(&mut self, input: &str) -> ForthResult {
        let mut new_code = Forth::into_code(input);
        self.code.append(&mut new_code);
        self.run()
    }

    fn run(&mut self) -> ForthResult {
        while let Some(term) = self.code.pop_front() {
            self.step_term(term)?;
        }
        Forth::ok()
    }

    fn step_term(&mut self, term: Term) -> ForthResult {
        match term {
            Term::Number(value) => self.push(value),
            Term::Word(word) => self.step_word(&word),
            Term::StartDefinition => self.store_definition(),
            Term::EndDefinition => {
                eprintln!("`;` without preceding `:`");
                Err(Error::InvalidWord)
            }
        }
    }

    fn step_word(&mut self, word: &str) -> ForthResult {
        if let Some(def) = self.defs.get(word) {
            let mut def = def.clone();
            while let Some(term) = def.pop_front() {
                self.step_term(term)?;
            }
            Self::ok()
        } else {
            self.step_built_in(word)
        }
    }

    fn divide((a, b): (Value, Value)) -> StackResult<Value> {
        a.checked_div(b).ok_or_else(|| {
            eprintln!("Cannot divide {} by {}", a, b);
            Error::DivisionByZero
        })
    }

    fn step_built_in(&mut self, word: &str) -> ForthResult {
        match word {
            "+" => self.bin_op(|(a, b)| Ok(a + b)),
            "-" => self.bin_op(|(a, b)| Ok(a - b)),
            "*" => self.bin_op(|(a, b)| Ok(a * b)),
            "/" => self.bin_op(Self::divide),
            "dup" => self.pop().and_then(|a| self.push(a).and(self.push(a))),
            "drop" => self.pop().and(Forth::ok()),
            "swap" => self
                .pop_two()
                .and_then(|(a, b)| self.push(b).and(self.push(a))),
            "over" => self
                .pop_two()
                .and_then(|(a, b)| self.push(a).and(self.push(b)).and(self.push(a))),
            _ => {
                eprintln!("{} is undefined", word);
                Err(Error::UnknownWord)
            }
        }
    }

    fn store_definition(&mut self) -> ForthResult {
        let mut def = Code::new();

        loop {
            match self.code.pop_front() {
                Some(Term::EndDefinition) => break,
                Some(term) => def.push_back(term),
                None => return Err(Error::InvalidWord),
            }
        }

        if let Some(Term::Word(name)) = def.pop_front() {
            self.store_word(name, def)
        } else {
            Err(Error::InvalidWord)
        }
    }

    fn push(&mut self, value: Value) -> ForthResult {
        self.stack.push(value);
        Forth::ok()
    }

    fn pop(&mut self) -> StackResult<Value> {
        self.stack.pop().ok_or_else(|| {
            eprintln!("Stack underflow");
            Error::StackUnderflow
        })
    }

    fn pop_two(&mut self) -> StackResult<(Value, Value)> {
        self.pop().and_then(|b| self.pop().map(|a| (a, b)))
    }

    fn bin_op<F>(&mut self, op: F) -> ForthResult
    where
        F: FnOnce((Value, Value)) -> StackResult<Value>,
    {
        self.pop_two()
            .and_then(op)
            .and_then(|value| self.push(value))
    }

    fn store_word(&mut self, name: String, code: Code) -> ForthResult {
        let mut resolved_def = Code::new();
        for t in code.iter() {
            match t {
                Term::Number(_) => resolved_def.push_back(t.clone()),
                Term::Word(s) => {
                    if let Some(cs) = self.defs.get(s) {
                        resolved_def.append(&mut cs.clone());
                    } else {
                        resolved_def.push_back(t.clone());
                    }
                }
                _ => {
                    eprintln!("Nested definition in {}", name);
                    return Err(Error::InvalidWord);
                }
            }
        }
        self.defs.insert(name, resolved_def);
        Forth::ok()
    }

    fn into_code(input: &str) -> Code {
        input
            .split(|c: char| c.is_whitespace() || c.is_control())
            .map(Term::from_str)
            .filter(Result::is_ok)
            .map(Result::unwrap)
            .collect()
    }

    fn ok() -> ForthResult {
        Ok(())
    }
}
