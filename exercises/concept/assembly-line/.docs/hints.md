# Hints

## General

## 1. Calculate the production rate per hour

- Determining the success rate can be done through a [conditional statement](https://doc.rust-lang.org/stable/book/ch03-05-control-flow.html#if-expressions) or with [pattern matching](https://doc.rust-lang.org/stable/book/ch18-01-all-the-places-for-patterns.html#match-arms).
- As Rust only allows multiplication between values of the same type, some [type casting](https://doc.rust-lang.org/rust-by-example/types/cast.html) will have to be done.

## 2. Calculate the number of working items produced per minute

- Just like multiplication, division is only possible between numbers of the same type. By writing a number with a decimal point (e.g. `1.0` instead of `1`) we can write inline constants with a floating point type.
