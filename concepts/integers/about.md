# Integers

Integers are whole numbers. They have no factional or decimal part. They are represented in the machine as a pattern of binary bits.

Integers may or may not be signed. Signedness means reserving one of the bits of the number to indicate
whether or not that number is negative.

Integers all have a **bit width**, which is just the number of bits making up that number. This impacts
the maximum value which can be represented by that integer type.

For example, one of the most common integer types is a `u8`: an unsigned, 8-bit integer. You may
recognize this type as a single byte. This has a minimum value of 0 and a maximum value of 256.

Rust has 12 integer primitive types, broken out by bit width and signedness:

| Unsigned | Signed  |
| -------- | ------- |
| `u8`     | `i8`    |
| `u16`    | `i16`   |
| `u32`    | `i32`   |
| `u64`    | `i64`   |
| `u128`   | `i128`  |
| `usize`  | `isize` |

An integer's bit width is always indicated by the number after the `u` or `i`. `usize` and `isize` are special: they have the same width as a pointer on the target machine. This means that on a 64-bit architecture, `usize` and `isize` are 64 bits wide, but on a 32-bit architecture, they are 32 bits wide.

## Which should I use?

In general, if you just need to represent some number, use an `i32`: 32 bits has a wide enough range that it's relatively unlikely to overflow, and signed numbers have fewer surprises than unsigned numbers. `i32` is equivalent to a `long` in C, or an `int` in Java. If you use a numeric literal and don't explicitly annotate the type, it will default to `i32`.

If you're working with binary data, it's easiest to work with bytes: `&[u8]` or `Vec<u8>`.

Rust uses `usize` for size and indexing of collections. This is because a computer can only theoretically address `usize::MAX` bytes of memory, so no larger in-memory collections are possible. Therefore, `usize` can be useful if you're doing a lot of work with collections.

If your data doesn't fit into a 32-bit integer, consider broadening to a 64 or 128 bit integer. If your data doesn't fit into a 128-bit integer, then you can use a big integer type such as is provided by [`num-bigint`](https://github.com/rust-num/num-bigint).

16 bit integers are not very common, but can be useful when targeting certain microcontrollers.

If you know that there are no valid negative cases for your application, it is better to use an unsigned integer instead of a signed integer. Counting is a good example of this.

## Converting between integers

Rust has no implicit numeric conversions. If you need to cast between integer types, there are two basic strategies: the `as` keyword, and the `From` and `TryFrom` traits.

Using the `as` keyword is simple: `expr as Type`. However, there are a number of [caveats and subtleties](https://doc.rust-lang.org/nomicon/casts.html) that you need to keep in mind when using `as` casts.

Trait-based casting is slightly more involved, but safer: conversion traits are only implemented where they are safe. For example, [`i32`](https://doc.rust-lang.org/std/primitive.i32.html) implements `From<u8>`, `From<u16>`, `From<i8>`, and `From<i16>`: any value representable by any of these types is guaranteed to be representable in an `i32`. It can be used like `i32::from(expr)`, or `expr.into()`, where `expr` resolves to one of those types. `i32` also implements `TryFrom<u32>`, `TryFrom<u64>`, `TryFrom<u128>`, `TryFrom<usize>`, `TryFrom<i64>`, `TryFrom<i128>`, and `TryFrom<isize>`. These conversions will explicitly either succeed or overflow, depending on the value. To use these, you will need to import the `std::convert::TryFrom` or `std::convert::TryInto` traits as appropriate, and then use `i32::try_from(expr)` or `expr.try_into()`, where `expr` resolves to an appropriate type.

Using trait-based casting instead of `as` casting is generally preferred, particularly when converting from a wider to a narrower type. Trait-based casting is inconsequentially more expensive than `as` casting: using `as` for performance reasons is not worth it unless in an extremely hot loop.
