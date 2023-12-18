# Getting Started

These exercises lean on Test-Driven Development (TDD), but they're not
an exact match.

The following steps assume that you are in the same directory as the exercise.

You must have Rust installed.
Follow the [Installation chapter in the Rust book](https://doc.rust-lang.org/book/ch01-01-installation.html).
The [Rust language section](http://exercism.org/languages/rust)
section from exercism is also useful.

## Step 1

Run the test suite. It can be run with `cargo`, which is installed with Rust.

```sh
$ cargo test
```

This will compile the `hello-world` crate and run the test, which fails.

```sh
running 1 test
test hello_world ... FAILED

failures:

---- hello_world stdout ----
thread 'hello_world' panicked at 'assertion failed: `(left == right)`
(left: `"Hello, World!"`, right: `"Goodbye, Mars!"`)', tests/hello-world.rs:5

failures:
    hello_world

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured
```

### Understanding Test Failures

The `hello_world` failure states that it is expecting the value,
`"Hello, World!"`, to be returned from `hello()`.
The left side of the assertion (at line 5) should be equal to the right side.

```sh
---- hello_world stdout ----
thread 'hello_world' panicked at 'assertion failed: `(left == right)`
(left: `"Hello, World!"`, right: `"Goodbye, Mars!"`)', tests/hello-world.rs:5
```

### Fixing the Error

To fix it, open up `src/lib.rs` and change the `hello` function to return
`"Hello, World!"` instead of `"Goodbye, Mars!"`.

```rust
pub fn hello() -> &'static str {
    "Hello, World!"
}
```

## Step 2

Run the test again. This time, it will pass.

```sh
running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured

     Running target/debug/deps/hello_world-bd1f06dc726ef14f

running 1 test
test hello_world ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured

   Doc-tests hello-world

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured
```

## Submit

Once the test is passing, you can submit your code with the following
command:

```sh
$ exercism submit
```
