# Macros

Macros are a powerful part of a Rust programmer's toolkit, and [macros by example](https://doc.rust-lang.org/reference/macros-by-example.html) are a relatively simple way to access this power. Let's write one!

## Context

What is a macro? [Wikipedia](https://en.wikipedia.org/wiki/Macro_(computer_science)) describes it thus:

> A macro (short for "macroinstruction", from Greek μακρός 'long') in computer science is a rule or pattern that specifies how a certain input sequence (often a sequence of characters) should be mapped to a replacement output sequence (also often a sequence of characters) according to a defined procedure. The mapping process that instantiates (transforms) a macro use into a specific sequence is known as macro expansion.

Illuminating! But to be more concrete, macros are a special syntax which allows you to generate code at compile time. Macros can be used for compile-time calculation, but more often they're just another way to abstract your code. For example, you've probably already used `println!()` and `vec![]`. These each take an arbitrary number of arguments, so you can't express them as simple functions. On the other hand, they always expand to some amount of absolutely standard Rust code. If you're interested, you can use the [cargo expand](https://github.com/dtolnay/cargo-expand) subcommand to view the results of macro expansion in your code.

For further information about macros in Rust, The Rust Book has a [good chapter](https://doc.rust-lang.org/book/ch19-06-macros.html) on them.

## Problem Statement

You can produce a `Vec` of arbitrary length inline by using the `vec![]` macro. However, Rust doesn't come with a way to produce a [`HashMap`](https://doc.rust-lang.org/std/collections/struct.HashMap.html) inline. Rectify this by writing a `hashmap!()` macro.

For example, a user of your library might write `hashmap!('a' => 3, 'b' => 11, 'z' => 32)`. This should expand to the following code:

```rust
{
   let mut hm = HashMap::new();
   hm.insert('a', 3);
   hm.insert('b', 11);
   hm.insert('z', 32);
   hm
}
```

Note that the [`maplit` crate](https://crates.io/crates/maplit) provides a macro which perfectly solves this exercise. Please implement your own solution instead of using this crate; please make an attempt on your own before viewing its source.

## Compatibility

Note that this exercise requires Rust 1.36 or later.


## Rust Installation

Refer to the [exercism help page][help-page] for Rust installation and learning
resources.

## Writing the Code

Execute the tests with:

```bash
$ cargo test
```

All but the first test have been ignored. After you get the first test to
pass, open the tests source file which is located in the `tests` directory
and remove the `#[ignore]` flag from the next test and get the tests to pass
again. Each separate test is a function with `#[test]` flag above it.
Continue, until you pass every test.

If you wish to run all ignored tests without editing the tests source file, use:

```bash
$ cargo test -- --ignored
```

To run a specific test, for example `some_test`, you can use:

```bash
$ cargo test some_test
```

If the specific test is ignored use:

```bash
$ cargo test some_test -- --ignored
```

To learn more about Rust tests refer to the [online test documentation][rust-tests]

Make sure to read the [Modules][modules] chapter if you
haven't already, it will help you with organizing your files.

## Further improvements

After you have solved the exercise, please consider using the additional utilities, described in the [installation guide](https://exercism.io/tracks/rust/installation), to further refine your final solution.

To format your solution, inside the solution directory use

```bash
cargo fmt
```

To see, if your solution contains some common ineffective use cases, inside the solution directory use

```bash
cargo clippy --all-targets
```

## Submitting the solution

Generally you should submit all files in which you implemented your solution (`src/lib.rs` in most cases). If you are using any external crates, please consider submitting the `Cargo.toml` file. This will make the review process faster and clearer.

## Feedback, Issues, Pull Requests

The [exercism/rust](https://github.com/exercism/rust) repository on GitHub is the home for all of the Rust exercises. If you have feedback about an exercise, or want to help implement new exercises, head over there and create an issue. Members of the rust track team are happy to help!

If you want to know more about Exercism, take a look at the [contribution guide](https://github.com/exercism/docs/blob/master/contributing-to-language-tracks/README.md).

[help-page]: https://exercism.io/tracks/rust/learning
[modules]: https://doc.rust-lang.org/book/ch07-02-defining-modules-to-control-scope-and-privacy.html
[cargo]: https://doc.rust-lang.org/book/ch14-00-more-about-cargo.html
[rust-tests]: https://doc.rust-lang.org/book/ch11-02-running-tests.html

## Source

Peter Goodspeed-Niklaus

## Submitting Incomplete Solutions
It's possible to submit an incomplete solution so you can see how others have completed the exercise.
