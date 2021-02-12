# Hints

## General

- [Rust Book: The Tuple Type](https://doc.rust-lang.org/book/ch03-02-data-types.html#the-tuple-type)
- [Rust By Example: Tuples](https://doc.rust-lang.org/stable/rust-by-example/primitives/tuples.html)
- [Rust By Example: Destructuring](https://doc.rust-lang.org/stable/rust-by-example/flow_control/match/destructuring.html)

## 1. Write a function `divmod` which returns both the quotient and remainder of a division

- Don't worry about optimizing for efficiency; the naive implementation is fine

## 2. Write an iterator adaptor `evens` which returns the even items from an arbitrary iterator

- Just chain together the suggested methods and everything will work out
- A number `n` is even if `n % 2 == 0`
- A closure is a function with abbreviated syntax: the argument name(s) go within a pair of `|` symbols, and the expression follows. Unlike normal functions, it is not always necessary to explicitly state the type of each argument, just the name. For example, here is how you can construct an iterator of odd squares: `(0..).map(|n| 2 * n + 1).map(|n| n * n)`.

## 3. Implement a `manhattan` method on a `Position` tuple struct

- Don't worry about method syntax; just replacing the `unimplemented` portion within the `impl Position` block will do the right thing.
- Consider that some values within a `Position` may be negative, but a distance is never negative.
- Calculating the absolute value is [built-in](https://doc.rust-lang.org/std/primitive.i16.html#method.abs)
