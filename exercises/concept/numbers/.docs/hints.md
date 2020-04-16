## General

- [cheats.rs - Basic Types](https://cheats.rs/#basic-types)
- [Rust book - Scalar types](https://doc.rust-lang.org/stable/book/ch03-02-data-types.html?highlight=primitive#scalar-types)
- [Rust reference - Integer literals](https://doc.rust-lang.org/stable/reference/tokens.html#integer-literals)
- [Rust reference - Floating point literals](https://doc.rust-lang.org/stable/reference/tokens.html#floating-point-literals)

## Calculate the production rate per second

- Determining the success rate can be done through a [conditional statement](https://doc.rust-lang.org/stable/book/ch03-05-control-flow.html#if-expressions) or with [pattern matching](https://doc.rust-lang.org/stable/book/ch18-01-all-the-places-for-patterns.html#match-arms).
- As Rust only allows multiplication between values of the same type, some [type casting](https://doc.rust-lang.org/rust-by-example/types/cast.html) will have to be done.

## Calculate the number of working items produced per second

- Just like multiplication, division is only possible between numbers of the same type. By writing a number with a decimal point (e.g. `1.0` instead of `1`) we can write inline constants with a floating point type.
