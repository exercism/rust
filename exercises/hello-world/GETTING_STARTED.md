# Getting Started

These exercises lean on Test-Driven Development (TDD), but they're not
an exact match.

The following steps assume that you are in the same directory as the exercise.

You must have rust installed.
Follow the [Installing Rust chapter in the Rust book](http://doc.rust-lang.org/stable/book/installing-rust.html).
The [Rust language section](http://exercism.io/languages/rust)
section from exercism is also useful.

## Step 1

Run the test suite. It can be run with `cargo`, which is installed with rust.

```
$ cargo test
```

This will fail, complaining that `hello-world` could not compile.

To fix this, create a new directory called `src`.
Create a new file called, `lib.rs`, inside the `src` directory.

## Step 2

Run the test again. It will give you a new error, another compile error.
Our `lib.rs` does not contain any code, specifically the `hello()`
function that our test is looking for.

### Fixing the Error

To fix it, open up the `src/lib.rs` file and add the following code:

```rust
pub fn hello(name: Option<&str>) -> String {
    "".to_string()
}
```

Our test is looking for the `hello()` function from the `hello_world`
crate. `lib.rs`, by default, is our crate root and our test
is looking for the `hello()` function there.

The code we are adding to `lib.rs` defines a public function (`pub fn`) that is called "hello".
The function accepts a `name` as an optional argument (`Option`).
The function returns a `String`.
We start by returning an empty string (`"".to_string()`).

## Step 3

Run the test again.

This time, code compilation will pass and we receive actual test failures.

```
running 3 tests
test test_other_same_name ... ignored
test test_sample_name ... ignored
test test_no_name ... FAILED

failures:

---- test_no_name stdout ----
thread 'test_no_name' panicked at 'assertion failed: `(left == right)`
(left: `"Hello, World!"`, right: `""`)', tests/hello-world.rs:5


failures:
    test_no_name

test result: FAILED. 0 passed; 1 failed; 2 ignored; 0 measured
```

### Understanding Test Failures

Only one of the tests runs (`test_no_name`) and it fails. The other
tests are ignored (more on that later).

The `test_no_name` failure states that it is expecting the value,
`"Hello, World!"`, to be returned from `hello("")`.
The left side of the assertion (at line 5) should be equal to the right side.

```
---- test_no_name stdout ----
thread 'test_no_name' panicked at 'assertion failed: `(left == right)`
(left: `"Hello, World!"`, right: `""`)', tests/hello-world.rs:5
```

To fix it, let's return `"Hello, World!"`, instead of an empty string
(`""`) inside our `hello` function.

```rust
pub fn hello(name: Option<&str>) -> String {
    "Hello, World!".to_string()
}
```

## Step 4

Run the test again. This time, it will pass.

```
running 3 tests
test test_other_same_name ... ignored
test test_sample_name ... ignored
test test_no_name ... ok

test result: ok. 1 passed; 0 failed; 2 ignored; 0 measured

   Doc-tests hello-world

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured
```

You may have noticed compiler warnings earlier:

```
Compiling hello-world v0.0.0
(file:////exercism/exercises/rust/hello-world)
src/lib.rs:1:14: 1:18 warning: unused variable: `name`, #[warn(unused_variables)] on by default
src/lib.rs:1 pub fn hello(name: Option<&str>) -> String {
                          ^~~~
```

Our `hello` function does not use the `name` argument so the
compiler is letting us know that we could potentially remove the
argument from our function (it likes "clean code").

As we make the rest of the tests pass, we will find that we need the `name`
argument, so don't delete it.

Activate the next test. Open the `tests/hello-world.rs` file.
Delete the `#[ignore]` line for the `test_sample_name` test.

## Step 5

Run the test suite again. Read the test failure and make the test pass.

As a reminder, the [rust book](http://doc.rust-lang.org/stable/book/README.html)
is a good reference for understanding rust.
The cargo output may also have hints to help you, depending on the errors you get.
For example, `rustc --explain E0425` will explain unresolved name errors.

## Wash, Rinse, Repeat

Delete one `#[ignore]`  at a time, and make each test pass before you move to
the next one.

## Submit

When everything is passing, you can submit your code with the following
command:

```
$ exercism submit src/lib.rs
```
