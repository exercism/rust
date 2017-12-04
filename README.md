# Exercism Rust Track

[![Build Status](https://travis-ci.org/exercism/rust.svg?branch=master)](https://travis-ci.org/exercism/rust)
[![Join the chat at https://gitter.im/exercism/rust](https://badges.gitter.im/exercism/rust.svg)](https://gitter.im/exercism/rust?utm_source=badge&utm_medium=badge&utm_campaign=pr-badge&utm_content=badge)

Exercism exercises in Rust

## Contributing

Thank you so much for contributing! :tada:

Please read about how to [get involved in a track](https://github.com/exercism/docs/tree/master/contributing-to-language-tracks). Be sure to read the Exercism [Code of Conduct](https://github.com/exercism/exercism.io/blob/master/CODE_OF_CONDUCT.md).

We welcome pull requests of all kinds. No contribution is too small.

We encourage contributions that provide fixes and improvements to existing exercises. Please note that this track's exercises must conform to the Exercism-wide standards described in the [documentation](https://github.com/exercism/docs/tree/master/language-tracks/exercises). If you're unsure about how to make a change, then go ahead and open a GitHub issue, and we'll discuss it.

## Exercise Tests

At the most basic level, Exercism is all about the tests. You can read more about how we think about test suites in [the Exercism documentation](https://github.com/exercism/docs/blob/master/language-tracks/exercises/anatomy/test-suites.md).

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

## Opening an Issue

If you plan to make significant or breaking changes, please open an issue so we can discuss it first. If this is a discussion that is relevant to more than just the Rust track, please open an issue in [exercism/discussions](https://github.com/exercism/discussions/issues).

## Submitting a Pull Request

Pull requests should be focused on a single exercise, issue, or conceptually cohesive change. Please refer to Exercism's [pull request guidelines](https://github.com/exercism/docs/blob/master/contributing/pull-request-guidelines.md).

Please follow the coding standards for Rust.  [rustfmt](https://github.com/nrc/rustfmt) may help with this
and can be installed with `cargo install rustfmt`.

### Verifying your Change

Before submitting your pull request, you'll want to verify the changes in two ways:

* Run all the tests for the Rust exercises
* Run an Exercism-specific linter to verify the track

All the tests for Rust exercises can be run from the top level of the repo with `_test/check-exercises.sh`. If you are on a Windows machine, there are additional [Windows-specific instructions](_test/WINDOWS_README.md) for running this.

## Contributing a New Exercise

Please see the documentation about [adding new exercises](https://github.com/exercism/docs/blob/master/you-can-help/make-up-new-exercises.md).

Note that:

- The simplest way to generate a project template is to run `bin/init_exercise.py`. You'll need a Python installation >= 3.5 in order to run this script, but it will automate most of the following points for you.

- Each exercise must stand on its own. Do not reference files outside the exercise directory. They will not be included when the user fetches the exercise.

- Exercises must conform to the Exercism-wide standards described in [the documentation](https://github.com/exercism/docs/tree/master/language-tracks/exercises).

- Each exercise should have:

      exercises/exercise-name/
                              tests/exercise-name.rs  <- a test suite
                              src/lib.rs              <- an empty file or with exercise stubs
                              example.rs              <- example solution that satisfies tests
                              Cargo.toml              <- with version equal to exercise defintion
                              Cargo.lock              <- Auto generated
                              README.md               <- Instructions for the exercise (see notes below)

- The stub file and test suite should use only the Rust core libraries. `Cargo.toml` should not list any external dependencies as we don't want to make the student assume required crates. If an `example.rs` uses external crates, include `Cargo-example.toml` so that `_tests/check-exercises.sh` can compile with these when testing.

- Except in extraordinary circumstances, the stub file should compile under `cargo test --no-run`.
  This allows us to check that the signatures in the stub file match the signatures expected by the tests.
  Use `unimplemented!()` as the body of each function to achieve this.
  If there is a justified reason why this is not possible, instead include a `.meta/ALLOWED_TO_NOT_COMPILE` containing the reason.

- If porting an existing exercise from problem-specifications that has a `canonical-data.json` file, use the version in `canonical-data.json` for that exercise as your `Cargo.toml` version.  Otherwise, use "0.0.0".

- An exercise may contain `.meta/hints.md`.  This is optional and will appear after the normal exercise
  instructions if present.  Rust is different in many ways from other languages.  This is a place where the differences required for Rust are explained.  If it is a large change, you may want to call this out as a comment at the top of `src/lib.rs`, so the user recognises to read this section before starting.

- If the test suite is appreciably sped up by running in release mode, and there is reason to be confident that the example implementation does not contain any overflow errors, consider adding a file `.meta/test-in-release-mode`. This should contain brief comments explaining the situation.

- If your exercise implements macro-based testing (see [#392](https://github.com/exercism/rust/issues/392#issuecomment-343865993) and [`perfect-numbers.rs`](https://github.com/exercism/rust/blob/master/exercises/perfect-numbers/tests/perfect-numbers.rs)), you will likely run afoul of a CI check which counts the `#[ignore]` lines and compares the result to the number of `#[test]` lines. To fix this, add a file `.meta/ignore-count-ignores` to disable that check for your exercise.

- `README.md` may be [regenerated](https://github.com/exercism/docs/blob/master/maintaining-a-track/regenerating-exercise-readmes.md) from Exercism data. The generator will use the `description.md` from the exercise directory in the [problem-specifications repository](https://github.com/exercism/problem-specifications/tree/master/exercises), then any hints in `.meta/hints.md`, then the [Rust-specific instructions](https://github.com/exercism/rust/blob/master/config/exercise-readme-insert.md). The `## Source` section comes from the `metadata.yml` in the same directory.  Convention is that the description of the source remains text and the link is both name and hyperlink of the markdown link.

- Be sure to add the exercise to an appropriate place in the `config.json` file.  The position in the file determines the order exercises are sent.   Generate a unique UUID for the exercise.  Current difficuly levels in use are 1, 4, 7 and 10.

## Rust icon
The Rust Logo is created by the Mozilla Corporation, and has been released under the [Creative Commons Attribution 4.0 International license](https://creativecommons.org/licenses/by/4.0/).
We tweaked the color from black to charcoal (#212121).
