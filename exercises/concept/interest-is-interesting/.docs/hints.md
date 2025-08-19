# Hints

## General

- [Floating-point types][floating-point-types] section on Chapter 3.2 Data Types
  of the Rust Book.

## 1. Calculate the interest rate

- By default, any floating-point number defined in Rust code is treated as a
  [`f64`][f64].
- To use [`f32`][f32] one can write numbers with a suffix of `_f32` or explicitly add
  the type to declaration.
  ```rust
    let x = 2.0_f32;

    let y: f32 = 3.0;
  ```
- [If statements][if-statements] can be used to return different values based on certain
  conditions.


## 2. Calculate the interest

- When calculating interest, it might be helpful to notice that `interest_rate` returns a percentage.

## 3. Calculate the annual balance update

- When calculating the annual balance update, we can use methods we have defined in previous steps.

## 4. Calculate the years before reaching the desired balance

- To calculate the years, one can keep looping until the desired balance is
  reached. You can use the [while-loop].

[while-loop]: https://doc.rust-lang.org/rust-by-example/flow_control/while.html
[f32]: https://doc.rust-lang.org/std/primitive.f32.html
[f64]: https://doc.rust-lang.org/std/primitive.f64.html
[if-statements]: https://doc.rust-lang.org/book/ch03-05-control-flow.html#if-expressions
[floating-point-types]:
    https://doc.rust-lang.org/book/ch03-02-data-types.html?highlight=floating#floating-point-types
[rust-book]: https://doc.rust-lang.org/book
