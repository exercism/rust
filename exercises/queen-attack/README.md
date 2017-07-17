# Queen Attack

Given the position of two queens on a chess board, indicate whether or not they
are positioned so that they can attack each other.

In the game of chess, a queen can attack pieces which are on the same
row, column, or diagonal.

A chessboard can be represented by an 8 by 8 array.

So if you're told the white queen is at (2, 3) and the black queen at
(5, 6), then you'd know you've got a set-up like so:

```plain
_ _ _ _ _ _ _ _
_ _ _ _ _ _ _ _
_ _ _ W _ _ _ _
_ _ _ _ _ _ _ _
_ _ _ _ _ _ _ _
_ _ _ _ _ _ B _
_ _ _ _ _ _ _ _
_ _ _ _ _ _ _ _
```

You'd also be able to answer whether the queens can attack each other.
In this case, that answer would be yes, they can, because both pieces
share a diagonal.

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

Make sure to read the [Crates and Modules](https://doc.rust-lang.org/stable/book/crates-and-modules.html) chapter if you
haven't already, it will help you with organizing your files.

## Feedback, Issues, Pull Requests

The [exercism/rust](https://github.com/exercism/rust) repository on GitHub is the home for all of the Rust exercises. If you have feedback about an exercise, or want to help implement new exercises, head over there and create an issue. Members of the [rust track team](https://github.com/orgs/exercism/teams/rust) are happy to help!

If you want to know more about Exercism, take a look at the [contribution guide](https://github.com/exercism/docs/blob/master/contributing-to-language-tracks/README.md).

[help-page]: http://exercism.io/languages/rust
[crates-and-modules]: http://doc.rust-lang.org/stable/book/crates-and-modules.html

## Source

J Dalbey's Programming Practice problems [http://users.csc.calpoly.edu/~jdalbey/103/Projects/ProgrammingPractice.html](http://users.csc.calpoly.edu/~jdalbey/103/Projects/ProgrammingPractice.html)

## Submitting Incomplete Solutions
It's possible to submit an incomplete solution so you can see how others have completed the exercise.
