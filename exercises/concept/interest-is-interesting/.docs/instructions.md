# Instructions

In this exercise you'll be working with savings accounts. Each year, the balance of your savings account is updated based on its interest rate.
The interest rate your bank gives you depends on the amount of money in your account (its balance):

- 3.213% for a negative balance (balance gets more negative).
- 0.5% for a positive balance less than `1000` dollars.
- 1.621% for a positive balance greater than or equal to `1000` dollars and less than `5000` dollars.
- 2.475% for a positive balance greater than or equal to `5000` dollars.

You have four tasks, each of which will deal your balance and its interest rate.

## 1. Calculate the interest rate

Implement the `interest_rate()` method to calculate the interest rate based on the specified balance:

```rust
interest_rate(200.75)
// 0.5
```


## 2. Calculate the interest

Implement the `interest()` method to calculate the interest based on the specified balance:

```rust
interest(200.75)
// 1.00375
```


## 3. Calculate the annual balance update

Implement the `annual_balance_update()` method to calculate the annual balance update, taking into account the interest rate:

```rust
annual_balance_update(200.75)
// 201.75375
```


## 4. Calculate the years before reaching the desired balance

Implement the `years_before_desired_balance()` method to calculate the minimum number of years required to reach the desired balance given annually compounding interest:

```rust
years_before_desired_balance(200.75, 214.88)
// 14
```

Note that the value returned is an integer.

~~~~exercism/note
When applying simple interest to a principal balance, the balance is multiplied by the interest rate and the product of the two is the interest amount.

Compound interest on the other hand is done by applying interest on a recurring basis.
On each application the interest amount is computed and added to the principal balance so that subsequent interest calculations are subject to a greater principal balance.
~~~~
