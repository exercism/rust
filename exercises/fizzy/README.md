# Fizzy

FizzBuzz is a children's game of counting through the integers. For each of them, if it's divisible by three, substitute the word "fizz"; if divisible by five, substitute "buzz"; if both, say both; if neither, say the number. It is not particularly difficult to implement, though it enjoyed some popularity for a time as [a quick way to tell whether entry-level programming applicants knew how to program _at all_](https://blog.codinghorror.com/why-cant-programmers-program/).

It has since fallen somewhat into disfavor for that task, because applicants began memorizing FizzBuzz implementations instead of learning to program.

We're going to do something more interesting than the basics: your task in this exercise is to implement FizzBuzz:

- with fully-customizable rules about what numbers produce what words
- fully generic on a very restricted minimal trait set
- such that it works just as well for the Collatz Sequence as for steadily increasing numbers
- with convenient helpers to make its use ergonomic

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

Peter Goodspeed-Niklaus [https://github.com/coriolinus/fizzy](https://github.com/coriolinus/fizzy)

## Submitting Incomplete Solutions
It's possible to submit an incomplete solution so you can see how others have completed the exercise.
