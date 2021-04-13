# About

Functions are bodies of code containing one or more [statements or expressions][statements or expressions]. A function may optionally
[return an value][return value].

Rust style is to use [snake case][snake case] for a function name, which is prefaced by the `fn` keyword for a function definition. For example

```rust
fn do_nothing() {}
```

The parentheses surround zero or more parameters, separated by commas. A parameter is a binding to a value of a particular type which is passed into the function.
Even if there are no parameters the parentheses are still required in the function definition. The actual value passed to a parameter is called
an argument.

The combination of parameters and return value is known as the function's signature. A function's signature requires that each parameter must
have its type annotated.

The curly braces enclose the body of the function: its statements and expressions. Even if the function contains no statements or expressions the
curly braces are still required for the function definition.

In the following example the function takes in one parameter of type `i32`, binds it to the name `value`, and prints it.

```rust
fn print_integer(value: i32) {
    println!("{:?}", value);
}
```

Note the parameter's definition. Each parameter is defined in the format `name: Type`.

A function can also return a value. By default, the output of the final expression is returned.
In the following example the function has one `i32` parameter and returns its double.

```rust
fn double_integer(value: i32) -> i32 {
    value * 2
}
```

The `-> i32` indicates that the function returns an `i32`. Unlike parameters, the returned value is not named.

It is possible to exit from a function early with the `return` keyword, like so:

```rust
fn long_function() -> i32 {
    let some_condition = false;
    // code snipped
    if (some_condition) {
        return 0;
    }
    // more code snipped
    42
}
```

`const fn` is used to define a [constant function][constant function], which can be evaluated at compile time.

```rust
const fn compute_data_checksum() -> u128 {
  const DATA: &[u8] = include_bytes!("my_big_data_file");
  // checksum implementation is left as an exercise for the reader
}

/// This checksum is used to validate that the user has not tampered with proprietary configuration.
pub const DATA_CHECKSUM: u128 = compute_data_checksum();
```

Because constant functions may be evaluated at compile time, they have some restrictions that normal functions do not.
In particular, a `const fn` can only call other functions also marked as `const`. Failure to abide by that restriction will result in a
compile error.

```rust
const fn multiply_integer(value: i32) -> i32 {
    use std::time::SystemTime;
    if SystemTime::now().elapsed().unwrap().as_nanos() == 0 { // this line errors
        value * 2
    } else {
        value * 3
    }
}
```

There are anonymous functions which are covered in the `closures` topic.

[statements or expressions]: https://doc.rust-lang.org/book/ch03-03-how-functions-work.html#function-bodies-contain-statements-and-expressions
[return value]: https://doc.rust-lang.org/book/ch03-03-how-functions-work.html#functions-with-return-values
[snake case]: https://en.wikipedia.org/wiki/Snake_case
[constant function]: https://doc.rust-lang.org/reference/const_eval.html#const-functions
