# Introduction

## Booleans

As in most other programming languages, a Boolean type in Rust has two possible values: `true` and `false`.

### Declaration
The Boolean type in Rust is specified using `bool`.

```rust
let t = true;

let f: bool = false; // with explicit type annotation
```

The main way to use Boolean values is through conditionals, such as an if
expression.
We’ll cover how if expressions work in Rust in later concepts.

### Use as a type
Booleans can be used as a type for a variable, function parameter or return
type.

```rust
let my_variable: bool;

fn my_function(input: bool) -> bool {
    // Do something
}

```

### Basic Operators
Rust supports following operators on `bool`:
- AND (`&&`) operator computes the logical AND of its operands.
  The result of `x && y` is `true` if both `x` and `y` evaluate to `true`.
  Otherwise, the result is `false`.
- OR (`||`) operator computes the logical OR of its operands.
  The result of `x || y` is `true` if either `x` or `y` evaluates to `true`.
  Otherwise, the result is `false`.
- NOT (`!`) operator computes logical negation of its operand.
  That is, it produces `true`, if the operand evaluates to `false`, and `false`, if the operand evaluates to `true`.

The following example demonstrates the effect of each operator on bool values:

```rust
let x = true;
let y = false;

!x // => false

x && y // => false

x || y // => true

```
