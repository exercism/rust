# Exercism Rust Track

[![CI](https://github.com/exercism/rust/workflows/CI/badge.svg?branch=main)](https://github.com/exercism/rust/actions?query=workflow%3ACI+branch%3Amain)
[![Join the chat at https://gitter.im/exercism/rust](https://badges.gitter.im/exercism/rust.svg)](https://gitter.im/exercism/rust?utm_source=badge&utm_medium=badge&utm_campaign=pr-badge&utm_content=badge)

Exercism exercises in Rust

## Contributing

Check out our [contributor documentation](docs/CONTRIBUTING.md).

## Exercise Tests

At the most basic level, Exercism is all about the tests. You can read more about how we think about test suites in [the Exercism documentation](https://github.com/exercism/legacy-docs/blob/main/language-tracks/exercises/anatomy/test-suites.md).

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

Pull requests should be focused on a single exercise, issue, or conceptually cohesive change. Please refer to Exercism's [pull request guidelines](https://github.com/exercism/legacy-docs/blob/main/contributing/pull-request-guidelines.md).

Please follow the coding standards for Rust.  [rustfmt](https://github.com/nrc/rustfmt) may help with this
and can be installed with `cargo install rustfmt`.

### Verifying your Change

Before submitting your pull request, you'll want to verify the changes in two ways:

* Run all the tests for the Rust exercises
* Run an Exercism-specific linter to verify the track

All the tests for Rust exercises can be run from the top level of the repo with `_test/check_exercises.sh`. If you are on a Windows machine, there are additional [Windows-specific instructions](_test/WINDOWS_README.md) for running this.

### On modifying the exercises' README

Please note that the README of every exercise is formed using several templates, not all of which are necessarily present on this repo. The most important of these:

- The `description.md` file in the exercise directory from the [problem-specifications repository](https://github.com/exercism/problem-specifications/tree/main/exercises)

- The `.meta/hints.md` file in the exercise directory on this repository

- The [Rust-specific instructions](https://github.com/exercism/rust/blob/main/exercises/shared/.docs/tests.md)

If you are modifying the section of the README that belongs to the template not from this repository, please consider [opening a PR](https://github.com/exercism/problem-specifications/pulls) on the `problem-specifications` repository first.

## Contributing a New Exercise

Please see the documentation about [adding new exercises](https://github.com/exercism/legacy-docs/blob/main/you-can-help/make-up-new-exercises.md).

Note that:

- The simplest way to generate, update or configure an exercise is to use the [exercise](https://github.com/exercism/rust/tree/main/util/exercise) utility provided in this repository. To compile the utility you can use the [bin/build_exercise_crate.sh](https://github.com/exercism/rust/tree/main/bin/build_exercise_crate.sh) script or, if the script does not work for you, use the `cargo build --release` command in the `util/exercise/` directory and then copy the `exercise` binary from the `util/exercise/target/release/` directory into the `bin/` directory. Use `bin/exercise --help` to learn about the existing commands and their possible usage.

- Each exercise must stand on its own. Do not reference files outside the exercise directory. They will not be included when the user fetches the exercise.

- Exercises must conform to the Exercism-wide standards described in [the documentation](https://github.com/exercism/legacy-docs/tree/main/language-tracks/exercises).

- Each exercise should have:

      exercises/exercise-name/
                              tests/exercise-name.rs  <- a test suite
                              src/lib.rs              <- an empty file or with exercise stubs
                              example.rs              <- example solution that satisfies tests
                              Cargo.toml              <- with version equal to exercise definition
                              Cargo.lock              <- Auto generated
                              README.md               <- Instructions for the exercise (see notes below)

- The stub file and test suite should use only the Rust core libraries. `Cargo.toml` should not list any external dependencies as we don't want to make the student assume required crates. If an `example.rs` uses external crates, include `Cargo-example.toml` so that `_tests/check_exercises.sh` can compile with these when testing.

- Except in extraordinary circumstances, the stub file should compile under `cargo test --no-run`.
  This allows us to check that the signatures in the stub file match the signatures expected by the tests.
  Use `unimplemented!()` as the body of each function to achieve this.
  If there is a justified reason why this is not possible, instead include a `.meta/ALLOWED_TO_NOT_COMPILE` containing the reason.

- If porting an existing exercise from problem-specifications that has a `canonical-data.json` file, use the version in `canonical-data.json` for that exercise as your `Cargo.toml` version.  Otherwise, use "0.0.0".

- An exercise may contain `.meta/hints.md`.  This is optional and will appear after the normal exercise
  instructions if present.  Rust is different in many ways from other languages.  This is a place where the differences required for Rust are explained.  If it is a large change, you may want to call this out as a comment at the top of `src/lib.rs`, so the user recognizes to read this section before starting.

- If the test suite is appreciably sped up by running in release mode, and there is reason to be confident that the example implementation does not contain any overflow errors, consider adding a file `.meta/test-in-release-mode`. This should contain brief comments explaining the situation.

- If your exercise implements macro-based testing (see [#392](https://github.com/exercism/rust/issues/392#issuecomment-343865993) and [`perfect-numbers.rs`](https://github.com/exercism/rust/blob/main/exercises/practice/perfect-numbers/tests/perfect-numbers.rs)), you will likely run afoul of a CI check which counts the `#[ignore]` lines and compares the result to the number of `#[test]` lines. To fix this, add a file `.meta/ignore-count-ignores` to disable that check for your exercise.

- `README.md` may be [regenerated](https://github.com/exercism/legacy-docs/blob/main/maintaining-a-track/regenerating-exercise-readmes.md) from Exercism data. The generator will use the `description.md` from the exercise directory in the [problem-specifications repository](https://github.com/exercism/problem-specifications/tree/main/exercises), then any hints in `.meta/hints.md`, then the [Rust-specific instructions](https://github.com/exercism/rust/blob/main/exercises/shared/.docs/tests.md). The `## Source` section comes from the `metadata.yml` in the same directory.  Convention is that the description of the source remains text and the link is both name and hyperlink of the markdown link.

- Be sure to add the exercise to an appropriate place in the `config.json` file.  The position in the file determines the order exercises are sent.   Generate a unique UUID for the exercise.  Current difficulty levels in use are 1, 4, 7 and 10.
