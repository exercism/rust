# Instructions

In this exercise you'll be generating semi-structured log messages.

## 1. Emit semi-structured messages

You'll start with some stubbed functions and the following enum:

```rust
#[derive(Clone, PartialEq, Debug)]
pub enum LogLevel {
    Info,
    Warning,
    Error,
}
```

Your goal is to emit a log message as follows: `"[<LEVEL>]: <MESSAGE>"`.
You'll need to implement functions that correspond with log levels.

For example, the below snippet demonstrates an expected output for the `log` function.

```rust
log(LogLevel::Error, "Stack overflow")
// Returns: "[ERROR]: Stack overflow"
```

And for `info`:

```rust
info("Timezone changed")
// Returns: "[INFO]: Timezone changed"
```

Have fun!

## 2. Optional further practice

There is a feature-gated test in this suite.
Feature gates disable compilation entirely for certain sections of your program.
They will be covered later.
For now just know that there is a test which is only built and run when you use a special testing invocation:

```sh
cargo test --features add-a-variant
```

This test is meant to show you how to add a variant to your enum.
