# Parallel Letter Frequency

Count the frequency of letters in texts using parallel computation.

Parallelism is about doing things in parallel that can also be done
sequentially. A common example is counting the frequency of letters.
Create a function that returns the total frequency of each letter in a
list of texts and that employs parallelism.

# Parallel Letter Frequency in Rust

Learn more about concurrency in Rust here:

- [Concurrency](https://doc.rust-lang.org/book/second-edition/ch16-00-concurrency.html)

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

- [Benchmark tests](https://doc.rust-lang.org/book/benchmark-tests.html)

Learn more about nightly Rust:

- [Nightly Rust](https://doc.rust-lang.org/book/nightly-rust.html)
- [Rustup: Working with nightly](https://github.com/rust-lang-nursery/rustup.rs#working-with-nightly-rust)


## Rust Installation

Refer to the [exercism help page][help-page] for Rust installation and learning
resources.

## Writing the Code

Execute the tests with:

```bash
$ cargo test
```

All but the first test have been ignored.  After you get the first test to
pass, remove the ignore flag (`#[ignore]`) from the next test and get the tests
to pass again.  The test file is located in the `tests` directory.   You can
also remove the ignore flag from all the tests to get them to run all at once
if you wish.

Make sure to read the [Modules](https://doc.rust-lang.org/book/second-edition/ch07-00-modules.html) chapter if you
haven't already, it will help you with organizing your files.

## Feedback, Issues, Pull Requests

The [exercism/rust](https://github.com/exercism/rust) repository on GitHub is the home for all of the Rust exercises. If you have feedback about an exercise, or want to help implement new exercises, head over there and create an issue. Members of the [rust track team](https://github.com/orgs/exercism/teams/rust) are happy to help!

If you want to know more about Exercism, take a look at the [contribution guide](https://github.com/exercism/docs/blob/master/contributing-to-language-tracks/README.md).

[help-page]: http://exercism.io/languages/rust
[modules]: https://doc.rust-lang.org/book/second-edition/ch07-00-modules.html
[cargo]: https://doc.rust-lang.org/book/second-edition/ch14-00-more-about-cargo.html


## Submitting Incomplete Solutions
It's possible to submit an incomplete solution so you can see how others have completed the exercise.
