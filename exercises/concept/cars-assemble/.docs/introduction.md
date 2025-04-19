# Introduction

## Numbers

There are two different types of numbers in Rust:

- Integers: numbers with no digits behind the decimal separator (whole numbers). Examples are `-6`, `0`, `1`, `25`, `976` and `500000`.
- Floating-point numbers: numbers with zero or more digits behind the decimal separator. Examples are `-2.4`, `0.1`, `3.14`, `16.984025` and `1024.0`.

The two default numeric types in Rust are `i32` and `f64`. An `i32` is a 32-bit integer and a `f64` is a 64-bit floating-point number.

Arithmetic is done using the standard arithmetic operators. Numbers can be
compared using the standard numeric comparison operators and the equality (`==`)
and inequality (`!=`) operators.

## Assignment

The following syntax can be used to define a variable and assign a value to it.

```rust
let my_variable = 5;
```

The above defines a variable with value 5 which automatically makes its type as
`i32`. The value of this variable cannot be changed.
In Rust, `snake_case` is used to define the name of a variable.

To create a variable that can be assigned a different value one can use `mut`
keyword:

```rust
let mut my_variable = 5;
my_variable = 10;
```

In Rust, integers and floats are different types hence you cannot assign a
float to an integer variable and vice-versa. Not even if the values seem equal:

```rust
let my_int = 5;
let my_float = my_int; // => panic

let my_float2 = 5.0;
let my_int2 = my_float2; // => panic
```

### Constants
Like immutable variables, *constants* are values that are bound to a name and
are not allowed to change, but there are a few differences between constants
and variables.

- First, you aren’t allowed to use `mut` with constants.
- Constants aren’t just immutable by default—they’re always immutable.
- You declare constants using the `const` keyword instead of the `let` keyword, and the type of the value *must* be annotated.
- Constants can be declared in any scope, including the global scope, which makes them useful for values that many parts of code need to know about.
- The last difference is that constants may be set only to a constant expression, not the result of a value that could only be computed at runtime.

Here’s an example of a constant declaration:

```rust
const SECONDS_IN_A_MINUTE: u32 = 60;
```

In Rust, constants are defined using `CAPITAL_SNAKE_CASE`.

### Rust requires explicit conversion between types

- Using the `as` casting:
    ```rust
    let my_float: f64 = 50.0;
    let my_int = my_float as i32; // works as expected
    ```

- Using the `.into()` method:
    ```rust
    let my_int = 50;
    let my_float: f64 = my_int.into(); // works as expected
    ```

Note that this requires specifying the variable type explicitly.

As an `i32` has less precision than a `f64`, converting from an `i32` to a `f64` is safe and lossless. However, converting from a `f64` to an `i32` could mean losing data.

## Numeric Operators

Rust supports various operators on integers and floats.

Take a look at the following table that illustrates how each operator behaves with integers and floats.

| Symbol | Integer                 | Floating Point |
|--------|-------------------------|----------------|
| `+`    | Addition                | Addition       |
| `-`    | Subtraction             | Subtraction    |
| `*`    | Multiplication          | Multiplication |
| `/`    | Division*               | Division       |
| `%`    | Remainder**             | Remainder      |

\* Integer division rounds towards zero.

\*\* Rust uses a remainder defined with [truncating division].
Given `remainder = dividend % divisor`, the remainder will have the same sign as the dividend.

Note that both operands of an operator must be of same type.
e.g. For `+` operator, an integer cannot be added to a float and vice-versa.

Following example demonstrates usage for these operators.

```rust
let result_1 = 3 + 6;
// => 9
let result_2 = 5.5 - 1.25;
// => 4.25
let result_3 = -5 * 14;
// => -70
let result_4 = 14 / 3;
// => 4
let result_5 = 100 % 7;
// => 2
```

## If Statements

In this exercise you must conditionally execute logic. The most common way to do this in Rust is by using an `if/else` statement:

`if` expressions start with the keyword `if`, followed by a condition. In this
case, the condition checks whether or not the variable `number` has a value less
than `5`.


```rust
let number = 3;

if number < 5 {
    println!("condition was true");
} else {
    println!("condition was false");
}
```

Optionally, we can also include an `else` expression, which we chose to do here,
to give the program an alternative block of code to execute should the condition
evaluate to `false`.

If you don’t provide an `else` expression and the condition is `false`, the program will just skip the `if` block and move on to the next bit of code.

The condition of an `if` statement must be of type `bool`. Rust has no concept of
_truthy_ values.

```rust
let number = 3;

if number {
    println!("number was three");
}
```

The `if` condition evaluates to a value of `3` this time, and Rust throws an
error:

```txt
  |
4 |     if number {
  |        ^^^^^^ expected `bool`, found integer

For more information about this error, try `rustc --explain E0308`.
error: could not compile `branches` due to previous error
```

You can use multiple conditions by combining `if` and `else` in an `else if` expression. For example:

```rust
let x = 6;

if x == 5 {
    // Execute logic if x equals 5
} else if x > 7 {
    // Execute logic if x greater than 7
} else {
    // Execute logic in all other cases
}
```


[truncating division]:
    https://en.wikipedia.org/wiki/Modulo_operation#Variants_of_the_definition
