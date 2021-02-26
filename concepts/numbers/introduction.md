# Introduction

In Rust, there are two main types of numbers availble for use: Integers and Floating-Point Types.

## Integers
An Integer is a whole number without a fractional component. They come in a number of variants ranging from 8-bit to 128-bit. The more bits you use, the larger the number can be, however, this increases the memory used. Each variant can be Signed or Unsigned. A Signed number allows for negative values whereas Unsigned does not. You can declare an Integer like so:
```rust
// Declaring and assigning an integer with a size of 8 bits
let myInt: u8 = 5;
```

## Floating-Point Types
A floating-point number includes decimal points and there are two types available in Rust. An `f32` and `f64` - both referring to a float of 32 bits and a float of 64 bits. `f64` is the default type in Rust as working with either size can be done at about the same speed when using a modern CPU. You can declare a Floating-Point Type like so:
```rust
// Declaring and assigning a float with a size of 32 bits
let myDecimal: f32 = 1.5;
```
