# Decimal

Implement an arbitrary-precision `Decimal` class.

Floating point numbers are the most common representation of non-integer real numbers in computing, and they're a common standard defined by [IEEE 754](https://en.wikipedia.org/wiki/IEEE_754). They're very flexible and versatile, but they do have some limitations. Famously, in floating point arithmetic, [`0.1 + 0.2 != 0.3`](http://0.30000000000000004.com/).

The solution to this issue is to find another, lossless way to model arbitrary-precision non-integer reals. This may be less efficient in terms of memory or processing speed than floating point numbers; the goal is to provide exact results.

Despite `Decimal` being a custom type, we should still be able to treat them as numbers: the `==`, `<`, `>`, `+`, `-`, and `*` operators should all work as expected on Decimals. For expediency, you are not required to implement division, as arbitrary-precision division can very quickly get out of hand. (How do you represent arbitrary-precision `1/3`?)

In Rust, the way to get these operations on custom types is to implement the relevant traits for your custom object. In particular, you'll need to implement at least `PartialEq`, `PartialOrd`, `Add`, `Sub`, and `Mul`. Strictly speaking, given that the decimal numbers form a total ordering, you should also implement `Eq` and `Ord`, though those traits are not checked for by these tests.

# Note

It would be very easy to implement this exercise by using the [bigdecimal](https://crates.io/crates/bigdecimal) crate. Don't do that; implement this yourself.

# Hints

- Instead of implementing arbitrary-precision arithmetic from scratch, consider building your type on top of the [num_bigint](https://crates.io/crates/num-bigint) crate.
- You might be able to [derive](https://doc.rust-lang.org/book/2018-edition/appendix-03-derivable-traits.html) some of the required traits.
- `Decimal` is assumed to be a signed type. You do not have to create a separate unsigned type, though you may do so as an implementation detail if you so choose.

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
