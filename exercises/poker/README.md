# Poker

Pick the best hand(s) from a list of poker hands.

See [wikipedia](https://en.wikipedia.org/wiki/List_of_poker_hands) for an
overview of poker hands.

## Hints

- Ranking a list of poker hands can be considered a sorting problem.
- Rust provides the [sort](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.sort) method for `Vec<T> where T: Ord`.
- [`Ord` types](https://doc.rust-lang.org/std/cmp/trait.Ord.html) are form a [total order](https://en.wikipedia.org/wiki/Total_order): exactly one of `a < b`, `a == b`, or `a > b` must be true.
- Poker hands do not conform to a total order: it is possible for two hands to be non-equal but have equal sort order. Example: `3S 4S 5D 6H JH"`, `"3H 4H 5C 6C JD"`.
- Rust provides the [`PartialOrd` trait](https://doc.rust-lang.org/std/cmp/trait.PartialOrd.html) to handle the case of sortable things which do not have a total order. However, it doesn't provide a standard `sort` method for `Vec<T> where T: PartialOrd`. The standard idiom to sort a vector in this case is `your_vec.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::{Less|Equal|Greater}));`, depending on your needs. `
- You might consider implementing a type representing a poker hand which implements `PartialOrd`.


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

Inspired by the training course from Udacity. [https://www.udacity.com/course/viewer#!/c-cs212/](https://www.udacity.com/course/viewer#!/c-cs212/)

## Submitting Incomplete Solutions
It's possible to submit an incomplete solution so you can see how others have completed the exercise.
