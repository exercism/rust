# Instructions

Your electric utility company needs to compile a summary of all bills sent to a customer who has closed their account. Unfortunately, your billing software provides the "last monthly bill" and the "final bill" separately, yet in reality, both were combined into a single bill.

Given a list of bills, write a function that replaces the last two bills with a single bill equal to their sum.

You'll start with the following stubbed function signature:

```rust
pub fn fix_billing_summary(summary: Vec<i32>) -> Vec<i32> {
	todo!()
}
```

## Optional further practice

There is a feature-gated test in this suite.
Feature gates disable compilation entirely for certain sections of your program.
They will be covered later.
For now just know that there is a test which is only built and run when you use a special testing invocation:

```sh
cargo test --features generic
```

This test is meant to show how you can modify your function to accept more complicated bills.

Hint: replace the function signature with:

```rust
pub fn fix_billing_summary<T: std::ops::Add>(summary: Vec<T>) -> Vec<T> {
	todo!()
}
```
