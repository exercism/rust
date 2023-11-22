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

Run `just add-exercise` and you'll be prompted for the minimal
information required to generate the exercise stub for you.
After that, jump in the generated exercise and fill in any todos you find.
This includes most notably:

- adding an example solution in `.meta/example.rs`
- Adjusting `.meta/test_template.tera`

The tests are generated using the template engine [Tera].
The input of the template is the canonical data from [`problem-specifications`].
if you want to exclude certain tests from being generated,
you have to set `include = false` in `.meta/tests.toml`.
Please include a comment about the reason for excluding it.
If you want to add additional track-specific tests, you can do so in a file `.meta/additional-tests.json`.
Only do this with a good reason for not upstreaming these tests.

Find some tips about writing tera templates [here](#tera-templates).

[Tera]: https://keats.github.io/tera/docs/
[`problem-specifications`]: https://github.com/exercism/problem-specifications/

Many aspects of a correctly implemented exercises are checked in CI.
I recommend that instead of spending lots of time studying and writing
documentation about the process, _just do it_.
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

Run `just update-exercise` to update an exercise.
This outsources most work to `configlet sync --update`
and runs the test generator again.

When updaing an exercise that doesn't have a tera template yet,
a new one will be generated for you.
You will likely have to adjust it to some extent.

Find some tips about writing tera templates [in the next section](#tera-templates).

## Tera templates

The full documentation for tera templates is [here][tera-docs].
Following are some approaches that have worked for our specific needs.

You will likely want to look at the exercise's `canonical-data.json`
to see what structure your input data has.
You can use `bin/symlink_problem_specifications.sh` to have this data
symlinked into the actual exercise directory. Handy!

The name of the input property is different for each exercise.
The default template will be something like this:

```txt
let input = {{ test.input | json_encode() }};
```

You will have to add the specific field of input for this exercise, e.g.

```txt
let input = {{ test.input.integers | json_encode() }};
```

Some exercises may have error return values.
You can use an if-else to render something different,
depending on the structure of the data:

```txt
let expected = {% if test.expected is object -%}
    None
{%- else -%}
    Some({{ test.expected }})
{%- endif %};
```

If every test case needs to do some crunching of the inputs,
you can add utils functions at the top of the tera template.
See [`word-count`'s template][word-count-tmpl] for an example.

Some exercises have multiple functions that need to be implemented
by the student and therefore tested.
The canonical data contains a field `property` in that case.
The template also has access to a value `fn_names`,
which is an array of functions found in `lib.rs`.
So, you can construct if-else-chains based on `test.property`
and render a different element of `fn_names` based on that.
See [`variable-length-quantity`'s template][var-len-q-tmpl] for an example.

There is a custom tera fiter `to_hex`, which formats ints in hexadecimal.
Feel free to add your own in the crate `rust-tooling`.
Custom filters added there will be available to all templates.
How to create such custom filters is documented int he [tera docs][tera-docs-filters].

[tera-docs]: https://keats.github.io/tera/docs/#templates
[word-count-tmpl]: /exercises/practice/word-count/.meta/test_template.tera
[var-len-q-tmpl]: /exercises/practice/variable-length-quantity/.meta/test_template.tera
[tera-docs-filters]: https://keats.github.io/tera/docs/#filters

## Syllabus

The syllabus is currently deactivated due to low quality.
see [this forum post](https://forum.exercism.org/t/feeling-lost-and-frustrated-in-rust/4882)
for some background on the desicion.

Creating a better syllabus would be very benefitial,
but it's a lot of work and requires good communication and coordination.
Make sure to discuss any plans you have on the forum
with people who have experience building syllabi.
