use std::error;
use std::fmt;

pub struct BankAccount {
    is_open: bool,
    balance: i32,
}
#[derive(Debug, PartialEq)]
pub enum BankAccountError {
    IsClosedError,
    WithdrawError,
    NegativeAmountError,
}

impl fmt::Display for BankAccountError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use BankAccountError::*;
        match *self {
            IsClosedError => write!(f, "IsClosedError"),
            WithdrawError => write!(f, "WithdrawError"),
            NegativeAmountError => write!(f, "NegativeAmountError"),
        }
    }
}

impl error::Error for BankAccountError {
    fn description(&self) -> &str {
        use BankAccountError::*;
        match *self {
            IsClosedError => "Cannot perform operation against a closed account",
            WithdrawError => "Cannot withdraw more money than is currently in the account",
            NegativeAmountError => "Amount for deposit or withdraw must not be negative",
        }
    }
}

impl BankAccount {
    pub fn new() -> Self {
        BankAccount {
            is_open: true,
            balance: 0,
        }
    }

    pub fn open(&mut self) {
        self.is_open = true;
    }

    pub fn close(&mut self) {
        self.is_open = false;
    }

    pub fn is_open(&self) -> bool {
        self.is_open
    }

    pub fn get_balance(&self) -> i32 {
        self.balance
    }

    pub fn deposit(&mut self, amount: i32) -> Result<i32, BankAccountError> {
        self.check_restrictions(amount)?;
        self.balance += amount;
        Ok(self.balance)
    }

    pub fn withdraw(&mut self, amount: i32) -> Result<i32, BankAccountError> {
        self.check_restrictions(amount)?;
        if amount > self.balance {
            return Err(BankAccountError::WithdrawError);
        }
        self.balance -= amount;
        Ok(self.balance)
    }

    fn check_restrictions(&self, amount: i32) -> Result<(), BankAccountError> {
        if !self.is_open {
            return Err(BankAccountError::IsClosedError);
        }
        if amount < 0 {
            return Err(BankAccountError::NegativeAmountError);
        }
        Ok(())
    }
}
