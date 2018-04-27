extern crate bank_account;
use bank_account::*;

#[test]
fn test_new_bank_account() {
    let account = BankAccount::new();
    assert!(account.is_open());
    assert_eq!(account.get_balance(), 0);
}

#[test]
#[ignore]
fn test_error_descriptions() {
    use std::error::Error;
    assert_eq!(
        BankAccountError::WithdrawError.description(),
        "Cannot withdraw more money than is currently in the account"
    );
    assert_eq!(
        BankAccountError::NegativeAmountError.description(),
        "Amount for deposit or withdraw must not be negative"
    );
    assert_eq!(
        BankAccountError::IsClosedError.description(),
        "Cannot perform operation against a closed account"
    );

}

#[test]
#[ignore]
fn test_error_fmt() {
    assert_eq!(
        format!("{:?}", BankAccountError::WithdrawError),
        "WithdrawError".to_string()
    );
    assert_eq!(
        format!("{:?}", BankAccountError::IsClosedError),
        "IsClosedError".to_string()
    );
    assert_eq!(
        format!("{:?}", BankAccountError::NegativeAmountError),
        "NegativeAmountError".to_string()
    );

}

#[test]
#[ignore]
fn test_close_and_open() {
    let mut account = BankAccount::new();
    assert!(account.is_open());
    account.close();
    assert!(!account.is_open());
    account.open();
    assert!(account.is_open());
}

#[test]
#[ignore]
fn test_action_on_closed_account() {
    let mut account = BankAccount::new();
    account.close();
    assert_eq!(account.deposit(1), Err(BankAccountError::IsClosedError));
    assert_eq!(account.withdraw(1), Err(BankAccountError::IsClosedError));
}

#[test]
#[ignore]
fn test_negative_amount() {
    let mut account = BankAccount::new();
    assert_eq!(
        account.deposit(-1),
        Err(BankAccountError::NegativeAmountError)
    );
    assert_eq!(
        account.withdraw(-1),
        Err(BankAccountError::NegativeAmountError)
    );

    account.close();
    assert_eq!(account.deposit(-1), Err(BankAccountError::IsClosedError));
    assert_eq!(account.withdraw(-1), Err(BankAccountError::IsClosedError));
}

#[test]
#[ignore]
fn test_deposit() {
    let mut bank_account = BankAccount::new();
    assert_eq!(bank_account.deposit(100), Ok(100));
    assert_eq!(bank_account.deposit(50), Ok(150));
    assert_eq!(bank_account.deposit(0), Ok(150));
}

#[test]
#[ignore]
fn test_withdraw() {
    let mut bank_account = BankAccount::new();
    assert_eq!(bank_account.deposit(100), Ok(100));

    assert_eq!(bank_account.withdraw(40), Ok(60));
    assert_eq!(bank_account.withdraw(0), Ok(60));
    assert_eq!(
        bank_account.withdraw(70),
        Err(BankAccountError::WithdrawError)
    );

    assert_eq!(bank_account.withdraw(60), Ok(0));
    assert_eq!(
        bank_account.withdraw(10),
        Err(BankAccountError::WithdrawError)
    );
}
