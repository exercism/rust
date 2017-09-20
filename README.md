# Exercism Rust Track

[![Build Status](https://travis-ci.org/exercism/rust.svg?branch=master)](https://travis-ci.org/exercism/rust)
[![Join the chat at https://gitter.im/exercism/rust](https://badges.gitter.im/exercism/rust.svg)](https://gitter.im/exercism/rust?utm_source=badge&utm_medium=badge&utm_campaign=pr-badge&utm_content=badge)

Exercism exercises in Rust

## Contributing

Thank you so much for contributing! :tada:

Please read about how to [get involved in a track](https://github.com/exercism/docs/tree/master/contributing-to-language-tracks). Be sure to read the Exercism [Code of Conduct](https://github.com/exercism/exercism.io/blob/master/CODE_OF_CONDUCT.md).

We welcome pull requests of all kinds. No contribution is too small.

We encourage contributions that provide fixes and improvements to existing exercises. Please note that this track's exercises must conform to the standards determined in the [exercism/problem-specifications](https://github.com/exercism/problem-specifications) repo. Changes to the tests or documentation of a common exercise will often warrant a PR in that repo before it can be incorporated into this track's exercises. If you're unsure, then go ahead and open a GitHub issue, and we'll discuss the change.

## Exercise Tests ##

At the most basic level, Exercism is all about the tests. They drive the user's implementation forward and tell them when the exercise is complete.

The utmost care and attention should be used when adding or making changes to the tests for an exercise. When implementing an exercise test suite, we want to provide a good user experience for the people writing a solution to the exercise. People should not be confused or overwhelmed.

We simulate Test-Driven Development (TDD) by implementing the tests in order of increasing complexity. We try to ensure that each test either

- helps triangulate a solution to be more generic, or
- requires new functionality incrementally.

Test files should use the following format:

```
extern crate exercise_name;

use exercise_name::*;

#[test]
fn test_descriptive_name() {
    assert_eq!(exercise_function(1), 1);
}

#[test]
#[ignore]
fn test_second_and_past_tests_ignored() {
    assert_ne!(exercise_function(1), 2);
}
```

## Submitting a Pull Request ##

Please keep the following in mind:

- Pull requests should be focused on a single exercise, issue, or change.

- We welcome changes to code style, and wording. Please open a separate PR for these changes if possible.

- Please open an issue before creating a PR that makes significant (breaking) changes to an existing exercise or makes changes across many exercises. It is best to discuss these changes before doing the work. Discussions related to exercises that are not track specific can be found in [exercism/discussions](https://github.com/exercism/discussions/issues).

- Follow the coding standards for Rust.  [rustfmt](https://github.com/nrc/rustfmt) may help with this
and can be installed with `cargo install rustfmt`.

- Watch out for trailing spaces, extra blank lines, and spaces in blank lines.

- All the tests for Rust exercises can be run from the top level of the repo with `_test/check-exercises.sh` Please run this command before submitting your PR.  See [Windows Specific instructions](_test/WINDOWS_README.md) for running this.

- The Rust repository uses [Travis CI](https://travis-ci.org/exercism/rust/) to validate the build and automatically runs `bin/fetch-configlet && bin/configlet` to ensure the exercises are configured correctly.  The status of this testing will be indicated in your pull request.

## Contributing a New Exercise ##

- If a new exercise would potentially be suitable for a track other than the Rust track, it should be defined in [problem-specifications](https://github.com/exercism/problem-specifications/tree/master/exercises) so that other tracks may benefit from it.

- If the exercise is commonly defined, please make sure the exercise conforms to specifications in the [exercism/problem-specifications](https://github.com/exercism/problem-specifications) repo.  (Note if slight changes are needed for Rust specific issues, see `.meta/hints.md` below.)

- Each exercise must stand on its own. Do not reference files outside the exercise directory. They will not be included when the user fetches the exercise.

- Exercises should use only the Rust core libraries.  If you must use an external crate, see note below about `Cargo-example.toml`.  `Cargo.toml` should not have external dependencies as we don't want to make the user assume required crates.

- Each exercise should have:

---
    exercises/exercise-name/
                            tests/exercise-name.rs  <- a test suite
                            src/lib.rs              <- an empty file or with exercise stubs
                            example.rs              <- example solution that satisfies tests
                            Cargo.toml              <- with version equal to exercise defintion
                            Cargo.lock              <- Auto generated 
                            README.md               <- Instructions for the exercise (see notes below)
---

- Note: If publishing an existing exercise from problem-specifications, use the version `canonical-data.json` for that exercise as your `Cargo.toml` version.  Otherwise, use "0.0.0"

- An exercise may contain `.meta/hints.md`.  This is optional and will appear after the normal exercise
  instructions if present.  Rust is different in many ways from other languages.  This is a place where the differences required for Rust are explained.  If it is a large change, you may want to call this out as a comment at the top of `src/lib.rs`, so the user recognises to read this section before starting.

- If an `example.rs` uses external crates, include `Cargo-example.toml` so that `_tests/check-exercises.sh` can compile with these when testing.

- `README.md` may be regenerated from exercism data.  The top section above `## Rust Installation` should contain `description.md` from the exercise directory in the [problem-specifications repository.](https://github.com/exercism/problem-specifications/tree/master/exercises)  The `## Source` section comes from the `metadata.yml` in the same directory.  Convention is that the description of the source remains text and the link is both name and hyperlink of the markdown link.

- Be sure to add the exercise to an appropriate place in the `config.json` file.  The position in the file determines the order exercises are sent.   Generate a unique UUID for the exercise.  Current difficuly levels in use are 1, 4, 7 and 10.

## Rust icon
The Rust Logo is created by the Mozilla Corporation, and has been released under the [Creative Commons Attribution 4.0 International license](https://creativecommons.org/licenses/by/4.0/).
We tweaked the color from black to charcoal (#212121).
