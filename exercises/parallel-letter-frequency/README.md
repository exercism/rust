# Parallel Letter Frequency

Count the frequency of letters in texts using parallel computation.

Parallelism is about doing things in parallel that can also be done
sequentially. A common example is counting the frequency of letters.
Create a function that returns the total frequency of each letter in a
list of texts and that employs parallelism.

# Parallel Letter Frequency in Rust

Learn more about concurrency in Rust here:

- [Concurrency](https://doc.rust-lang.org/book/ch16-00-concurrency.html)

## Bonus

This exercise also includes a benchmark, with a sequential implementation as a
baseline. You can compare your solution to the benchmark. Observe the
effect different size inputs have on the performance of each. Can you
surpass the benchmark using concurrent programming techniques?

As of this writing, test::Bencher is unstable and only available on
*nightly* Rust. Run the benchmarks with Cargo:

```
cargo bench
```

If you are using rustup.rs:

```
rustup run nightly cargo bench
```

- [Benchmark tests](https://doc.rust-lang.org/stable/unstable-book/library-features/test.html)

Learn more about nightly Rust:

- [Nightly Rust](https://doc.rust-lang.org/stable/book/2018-edition/appendix-06-nightly-rust.html)
- [Rustup: Working with nightly](https://github.com/rust-lang-nursery/rustup.rs#working-with-nightly-rust)


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

## Submitting Incomplete Solutions
It's possible to submit an incomplete solution so you can see how others have completed the exercise.
