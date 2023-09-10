# Contributing to the Rust Exercism Track

Issues and pull requests are currently being auto-closed.
Please make a post on [the Exercism forum] to propose changes.
Contributions are very welcome if they are coordinated on the forum.

[the Exercism forum]: https://forum.exercism.org/

## General policies

- [Code of Conduct](https://exercism.org/code-of-conduct)
- [Exercism's PR guidelines](https://exercism.org/docs/community/being-a-good-community-member/pull-requests).

## Tooling

Some tooling is present as bash scripts in `bin/`.
A lot more is present in `rust-tooling/`,
which should be preferred for anything non-trivial.

There is also a [`justfile`](https://github.com/casey/just)
with a couple useful commands to interact with the repo.
Feel free to extend it.

If you want to run CI tests locally, `just test` will get you quite far.

## Creating a new exercise

Please familiarize yourself with the [Exercism documentation about practice exercises].

[Exercism documentation about practice exercises]: https://exercism.org/docs/building/tracks/practice-exercises

Run `just add-practice-exercise` and you'll be prompted for the minimal
information required to generate the exercise stub for you.
After that, jump in the generated exercise and fill in any todos you find.
This includes most notably:
- adding an example solution in `.meta/example.rs`
- Adjusting `.meta/test_template.tera`

The tests are generated using the template engine [Tera].
The input of the template is the canonical data from [`problem-specifications`].
if you want to exclude certain tests from being generated,
you have to set `include = false` in `.meta/tests.toml`.

[Tera]: https://keats.github.io/tera/docs/
[`problem-specifications`]: https://github.com/exercism/problem-specifications/

Many aspects of a correctly implemented exercises are checked in CI.
I recommend that instead of spending lots of time studying and writing
documentation about the process, *just do it*.
If something breaks, fix it and add a test / automation
so it won't happen anymore.

If you are creating a practice exercise from scratch,
one that is not present in `problem-specifications`,
you have to write your tests manually.
Tests should be sorted by increasing complexity,
so students can un-ignore them one by one and solve the exercise with TDD.
See [the Exercism documentation](https://github.com/exercism/legacy-docs/blob/main/language-tracks/exercises/anatomy/test-suites.md)
for more thoughts on writing good tests.

Except for extraordinary circumstances,
the stub file should compile under `cargo test --no-run`.
This allows us to check that the signatures in the stub file
match the signatures expected by the tests.
Use `todo!()` as the body of each function to achieve this.
If there is a justified reason why this is not possible,
include a `.custom."allowed-to-not-compile"` key
in the exercise's `.meta/config.json` containing the reason.

If your exercise implements macro-based testing
(see [#392](https://github.com/exercism/rust/issues/392#issuecomment-343865993)
and [`perfect-numbers.rs`](https://github.com/exercism/rust/blob/main/exercises/practice/perfect-numbers/tests/perfect-numbers.rs)),
you will likely run afoul of a CI check which counts the `#[ignore]` lines
and compares the result to the number of `#[test]` lines.
To fix this, add a marker to the exercise's `.meta/config.json`:
`.custom."ignore-count-ignores"` should be `true`
to disable that check for your exercise.

## Updating an exercise

Many exercises are derived from [`problem-specifications`].
This includes their test suite and user-facing documentation.
Before proposing changes here,
check if they should be made `problem-specifications` instead.

Run `just update-practice-exercise` to update an exercise.
This outsources most work to `configlet sync --update`
and runs the test generator again.

## Syllabus

The syllabus is currently deactivated due to low quality.
see [this forum post](https://forum.exercism.org/t/feeling-lost-and-frustrated-in-rust/4882)
for some background on the desicion.

Creating a better syllabus would be very benefitial,
but it's a lot of work and requires good communication and coordination.
Make sure to discuss any plans you have on the forum
with people who have experience building syllabi.
