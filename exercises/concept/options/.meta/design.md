# `Option`

## Learning Objectives

After completing the exercise, the user should:

- Know what the `Option` keyword refers to.
- Know that the `Some` and `None` keywords refer to the variants of the `Option` enum.
- Understand that `Option`s in Rust are the closest analog to null pointers in other languages.
- Understand that `Option`s are commonly paired with pattern matching in order to query for the presence or absence of a value.

## Out of Scope

- Using `if let` or `while let` syntax for matching on an `Option`.
- Methods on `Option`s.

## Concepts

The concepts this exercise unlocks are:

- Know what the `Option`, `Some`, and `None` keywords refer to.
- Know that `Option`s are often paired with pattern matching.
- Know that `Option`s are Rust's analog of null pointers in other languages.

## Prerequisites

- basics
- structs
- pattern matching with `match`

## Resources to Refer to

### Hints

- <https://doc.rust-lang.org/std/option/>
- <https://doc.rust-lang.org/rust-by-example/std/option.html>
- <https://learning-rust.github.io/docs/e3.option_and_result.html>
- [The Billion-Dollar Mistake](https://www.infoq.com/presentations/Null-References-The-Billion-Dollar-Mistake-Tony-Hoare/)

### After

Take a look at the list of methods implemented on the `Option` type:

- <https://doc.rust-lang.org/std/option/enum.Option.html>

Some commonly used methods on `Option`s include:

- `is_some` to check if an `Option` is of the `Some` variant
- `is_none` to check if an `Option` is of the `None` variant
- `map` in order to apply function to a value contained in an `Option`
- `take` in order to replace an `Option` with a `None`; useful when needing to swap two values

## Representer

No changes required.

## Analyzer

No changes required.
