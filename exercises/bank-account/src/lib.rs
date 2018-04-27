use std::error;
use std::fmt;

pub struct BankAccount {}

#[derive(Debug, PartialEq)]
pub enum BankAccountError {
}

impl fmt::Display for BankAccountError {
    fn fmt(&self, _: &mut fmt::Formatter) -> fmt::Result {
        unimplemented!()
    }
}

impl error::Error for BankAccountError {
    fn description(&self) -> &str {
        unimplemented!()
    }
}

impl BankAccount {
    pub fn new() -> Self {
        unimplemented!()
    }

    pub fn open(&mut self) {
        unimplemented!()
    }

    pub fn close(&mut self) {
        unimplemented!()
    }

    pub fn is_open(&self) -> bool {
        unimplemented!()
    }

    pub fn get_balance(&self) -> i32 {
        unimplemented!()
    }

    pub fn deposit(&mut self, _: i32) -> Result<i32, BankAccountError> {
        unimplemented!()
    }

    pub fn withdraw(&mut self, _: i32) -> Result<i32, BankAccountError> {
        unimplemented!()
    }
}
