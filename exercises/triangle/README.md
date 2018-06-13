# Triangle

Determine if a triangle is equilateral, isosceles, or scalene.

An _equilateral_ triangle has all three sides the same length.

An _isosceles_ triangle has at least two sides the same length. (It is sometimes
specified as having exactly two sides the same length, but for the purposes of
this exercise we'll say at least two.)

A _scalene_ triangle has all sides of different lengths.

## Note

For a shape to be a triangle at all, all sides have to be of length > 0, and
the sum of the lengths of any two sides must be greater than or equal to the
length of the third side. See [Triangle Inequality](https://en.wikipedia.org/wiki/Triangle_inequality).

## Dig Deeper

The case where the sum of the lengths of two sides _equals_ that of the
third is known as a _degenerate_ triangle - it has zero area and looks like
a single line. Feel free to add your own code/tests to check for degenerate triangles.

# Triangle in Rust

- [Result](https://doc.rust-lang.org/std/result/index.html)

Implementation of this can take many forms. Here are some topics that may help you, depending on the approach you take.

- [Enums](https://doc.rust-lang.org/book/second-edition/ch06-00-enums.html)
- [Traits](https://doc.rust-lang.org/book/second-edition/ch10-02-traits.html)
- [BTreeSet](https://doc.rust-lang.org/std/collections/btree_set/struct.BTreeSet.html)

Or maybe you will come up with an approach that uses none of those!


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

The Ruby Koans triangle project, parts 1 & 2 [http://rubykoans.com](http://rubykoans.com)

## Submitting Incomplete Solutions
It's possible to submit an incomplete solution so you can see how others have completed the exercise.
