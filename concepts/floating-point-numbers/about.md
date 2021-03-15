# Floating-point Numbers

Floating-point numbers are real numbers: they may have a fractional part. They are represented in the machine as a pattern of binary bits, using the [IEEE-754 specification](https://en.wikipedia.org/wiki/IEEE_754).
People refer to floating-point numbers as floats.

Floating point numbers are always signed. Signedness means reserving one of the bits of the number to indicate
whether or not that number is negative.

Floating point numbers have a **bit width**, which is just the number of bits making up that number. This impacts
the range and precision of values which can be represented by that type.

Rust has 2 floating-point primitive types: `f32` and `f64`. The number after the `f` indicates the bit width. In other languages,`f32` is sometimes referred to as "single-precision", and `f64` is sometimes referred to as "double-precision".

## Which should I use?

In general, use `f64`: it's as fast as `f32` on most modern consumer hardware, and significantly reduces the incidence of [floating-point inaccuracy](https://0.30000000000000004.com/).

If you need infinite-precision rational numbers, you might use the [`num-rational` crate](https://crates.io/crates/num-rational), which provides a `BigRational` type. If you need fixed-precision decimal numbers, you might use the [`rust_decimal` crate](https://crates.io/crates/rust_decimal), which provides a `Decimal` type.

## Converting between floating-point numbers

Rust has no implicit numeric conversions. If you need to cast between float types, there are two basic strategies: the `as` keyword, and the `From` and `TryFrom` traits.

Using the `as` keyword is simple: `expr as Type`. However, there are a number of [caveats and subtleties](https://doc.rust-lang.org/nomicon/casts.html) that you need to keep in mind when using `as` casts.

Trait-based casting is slightly more involved, but safer: conversion traits are only implemented where they are safe. For example, [`f32`](https://doc.rust-lang.org/std/primitive.f32.html) implements `From<u8>`, `From<u16>`, `From<i8>`, and `From<i16>`: any value representable by any of these types is guaranteed to be representable in an `f32`. It can be used like `f32::from(expr)`, or `expr.into()`, where `expr` resolves to one of those types.

When converting floating-point values `as`-casting is often preferred simply because of the relative scarcity of trait-based cast implementations. As of Oct 2020, `TryFrom` is not implemented for floating-point numbers. `as`-casting from `f32` to `f64` is lossless. The reverse is lossy, but has a defined casting protocol intended to minimize loss.
