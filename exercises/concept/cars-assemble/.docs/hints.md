# Hints

## General

- [Rust book][rust-book-data-types] on Integers and Floating point numbers.

## 1. Calculate the success rate

- Determining the success rate can be done through a [conditional statement][if-statement].

## 2. Calculate the production rate per second

- Use `.into()` to convert `i32` to `f64`.
- Use the `success_rate()` method you wrote earlier to determine the success rate.
- Rust does not allow for multiplication to be applied to two different number
  types (such as an `i32` and a `f64`). Both must be of the same type.
- Numbers can be compared using the built-in comparision and equality operators.
  Checkout [full list of operators][operators-list] supported by Rust.
## 3. Calculate the number of working items produced per second

- Use `.into()` to convert `i32` to `f64`.
- Rounding a float to a certain precision (here 1 decimal places) can be done using:
   ```rust
   let my_float = 3.166;
   let rounded = (my_float * 10.0).round() / 10.0;
   // => 3.2
   ```
   Read more on this [StackOverflow answer][stackoverflow-rounding].

[if-statement]: https://doc.rust-lang.org/rust-by-example/flow_control/if_else.html#ifelse
[stackoverflow-rounding]: https://stackoverflow.com/a/28656825
[operators-list]:https://doc.rust-lang.org/book/appendix-02-operators.html#operators
[rust-book-data-types]:
    https://doc.rust-lang.org/book/ch03-02-data-types.html#integer-types
