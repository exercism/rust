# Reverse String

Reverse a string

For example:
input: "cool"
output: "looc"

## Bonus
Test your function on this string: `uüu` and see what happens. Try to write a function that properly
reverses this string. Hint: grapheme clusters

To get the bonus test to run, remove the ignore flag (`#[ignore]`) from the
last test, and execute the tests with:

```bash
$ cargo test --features grapheme
```


## Rust Installation

Refer to the [exercism help page][help-page] for Rust installation and learning
resources.

## Writing the Code

Execute the tests with:

```bash
$ cargo test
```

All but the first test have been ignored. If you wish to run every possible test use:

```bash
$ cargo test -- --ignored
```

To run specific test, for example `some_test`, you can use:

```bash
$ cargo test some_test
```

To find test names, view the tests source file wich is located in `tests` directory.
Every function that contains `#[test]` flag above it's name is a separate test.
If the specfic test is ignored use:

```bash
$ cargo test some_test -- --ignored
```

Alternatively you can edit the tests source file and remove the `#[ignore]` flag
from the tests you wish to run and use the first `cargo test` command to run them.

Make sure to read the [Modules](https://doc.rust-lang.org/book/second-edition/ch07-00-modules.html) chapter if you
haven't already, it will help you with organizing your files.

## Feedback, Issues, Pull Requests

The [exercism/rust](https://github.com/exercism/rust) repository on GitHub is the home for all of the Rust exercises. If you have feedback about an exercise, or want to help implement new exercises, head over there and create an issue. Members of the rust track team are happy to help!

If you want to know more about Exercism, take a look at the [contribution guide](https://github.com/exercism/docs/blob/master/contributing-to-language-tracks/README.md).

[help-page]: http://exercism.io/languages/rust
[modules]: https://doc.rust-lang.org/book/second-edition/ch07-00-modules.html
[cargo]: https://doc.rust-lang.org/book/second-edition/ch14-00-more-about-cargo.html
[rust-tests]: https://doc.rust-lang.org/book/second-edition/ch11-02-running-tests.html

## Source

Introductory challenge to reverse an input string [https://medium.freecodecamp.org/how-to-reverse-a-string-in-javascript-in-3-different-ways-75e4763c68cb](https://medium.freecodecamp.org/how-to-reverse-a-string-in-javascript-in-3-different-ways-75e4763c68cb)

## Submitting Incomplete Solutions
It's possible to submit an incomplete solution so you can see how others have completed the exercise.
