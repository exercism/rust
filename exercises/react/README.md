# React

Implement a basic reactive system.

Reactive programming is a programming paradigm that focuses on how values
are computed in terms of each other to allow a change to one value to
automatically propagate to other values, like in a spreadsheet.

Implement a basic reactive system with cells with settable values ("input"
cells) and cells with values computed in terms of other cells ("compute"
cells). Implement updates so that when an input value is changed, values
propagate to reach a new stable system state.

In addition, compute cells should allow for registering change notification
callbacks.  Call a cell’s callbacks when the cell’s value in a new stable
state has changed from the previous stable state.

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
