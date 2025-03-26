pub fn interest_rate(balance: f64) -> f64 {
    if balance < 0.0 {
        3.213
    } else if balance < 1000.0 {
        0.5
    } else if balance < 5000.0 {
        1.621
    } else {
        2.475
    }
}

pub fn interest(balance: f64) -> f64 {
    balance * interest_rate(balance) / 100.0
}

pub fn annual_balance_update(balance: f64) -> f64 {
    balance + interest(balance)
}

pub fn years_before_desired_balance(balance: f64, target_balance: f64) -> u8 {
    let mut accumulating_balance = balance;
    let mut years = 0;

    while accumulating_balance < target_balance {
        accumulating_balance = annual_balance_update(accumulating_balance);
        years += 1;
    }

    years
}
