# Introduction

## Functions

Functions are a fundamental building block of Rust programs.
Let's look at a function to get an idea of what's going on:

```rust
fn add(x: i32, y: i32) -> i32 {
  x + y
}
```

The first keyword is `pub`, a visibility modifier.
It is optional and you will learn about it later, no need to worry about it now.

The keyword `fn` and the name of the function are mandatory.

A function may have zero or more parameters enclosed in parentheses `()`.
A parameter consist of its name and type, separated by a colon.
Multiple parameters are separated by commas.

The return type is specified after an arrow `->`.
It is omitted if the function doesn't return anything.
This is common when the function's purpose is to perform a side effect.
```rust
fn perform_side_effect() {
    // body
}
```

Lastly, the body of the function is enclosed in curly braces `{}`.
A function automatically returns the result of its last expression.
Simple functions like the one above can therefore consist of just an expression.

Invoking a function is done by specifying its name followed by parentheses.
If the function requires arguments, they must be specified within the parentheses.

```rust
add(2, 3) // evaluates to 5
```

## Integers and arithmetic

The parameters and the return value of the above function all have the same type: `i32`.
This is a 32-bit, signed integer.
You will learn more about what that means exactly later.
For now it's enough to know that an `i32` can store positive and negative *whole* numbers.

Rust provides several arithmetic operators to perform calculations.
The most common ones are:
- `+` for addition
- `-` for subtraction
- `*` for multiplication
- `/` for division (note that this rounds towards zero)

There are more, and you will learn about them later.
