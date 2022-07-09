# Introduction

## Numbers

There are two different categories of numbers in Rust: integers (which can be signed or unsigned) and floating-point numbers.

## Integers

- Integers: numbers with no digits behind the decimal separator (whole numbers).
  Integer types can either store only positive numbers (unsigned) or store either positive and negative numbers (signed).
  Examples are -6, 0, 1, 25, 976 and 500000.

## Floating-Point Numbers

- Floating-point numbers: numbers with zero or more digits behind the decimal separator.
  Examples are -2.4, 0.1, 3.14, 16.984025 and 1024.0.

## Naming numeric types

The name of a numeric type consists of two parts:

- A letter to specify whether it's an unsigned integer (u), signed integer (i), or floating-point number (f).
- A number to specify the type's size in bits. Larger types have a greater range between minimum and maximum values.
  For floating points it will also allow for more numbers behind the decimal separator.

The following combinations are possible:

- 8 bits: `u8`, `i8`
- 16 bits: `u16`, `i16`
- 32 bits: `u32`, `i32`, `f32`
- 64 bits: `u64`, `i64`, `f64`
- 128 bits: `u128`, `i128`

Note that there are only 32-bits and 64-bits variants for floating-point numbers.

## Converting between number types

Rust doesn't do any implicit type conversion.
This means that if you need to turn one numeric type into another, you have to do so explicitly.
When converting from a larger type to a smaller one (for instance `u64` to `u32`) you could lose data.
Converting from a floating point to an integer **will** lose everything behind the decimal point, effectively rounding down.
