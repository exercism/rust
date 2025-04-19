# Introduction

## Floating Point Numbers

A floating-point number is a number with zero or more digits behind the decimal
separator.
Examples are `-2.4`, `0.1`, `3.14`, `16.984025` and `1024.0`.

Different floating-point types can store different numbers of digits after the digit separator - this is referred to as its precision.

Rust implements the IEEE 754-2008 "binary32" and "binary64" floating-point types as `f32` and `f64`, respectively.
The f32 type is a single-precision float, and f64 has double precision.

- `f32`: 32 bit floating point precision. Written as `2.45_f32`.
- `f64`: 64 bit floating point precision. This is default in rust. Written as
  `2.45_f64`.

```rust
fn main() {
    let x = 2.0; // f64

    let y: f32 = 3.0; // f32
}
```

As can be seen, each type can store a different number of digits.
This means that trying to store PI in a `float` will only store the first 6 to 9 digits (with the last digit being rounded).

## Conditional Loops with while

In this exercise you may also want to use a loop.
There are several ways to write loops in Rust, but the `while` loop is most appropriate here:

```rust
let mut x = 10;

while x > 0 {
    // Execute logic if x > 10
    x = x - 1;
}
```

In the above example, we define a `while` loop with the condition `x > 0`.
Since the initial value of `x` was set to `10` and we decrement it by 1 every
loop, this will run the code inside the curly braces 10 times.
