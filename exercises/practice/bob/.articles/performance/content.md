# Performance

In this approach, we'll find out how to most efficiently determine the response for Bob in Rust.

The [approaches page][approaches] lists two idiomatic approaches to this exercise:

1. [Using `if` statements][approach-if].
2. [Using `match` on a `tuple`][approach-match].

For our performance investigation, we'll also include a third approach that [uses an answer array][approach-answer-array].

## Benchmarks

To benchmark the approaches, we wrote a [small benchmark application][benchmark-application].

```
test test_multiple_line_question_match  ... bench:          96 ns/iter (+/- 17)
test test_multiple_line_question_if     ... bench:          97 ns/iter (+/- 12)
test test_multiple_line_question_array  ... bench:         100 ns/iter (+/- 2)
```

All three approaches are close in performance, but the `if` statements and `match` approaches may be considered to be more idiomatic.

[approaches]: https://exercism.org/tracks/rust/exercises/bob/approaches
[approach-if]: https://exercism.org/tracks/rust/exercises/bob/approaches/if-statements
[approach-match]: https://exercism.org/tracks/rust/exercises/bob/approaches/match-on-tuple
[approach-answer-array]: https://exercism.org/tracks/rust/exercises/bob/approaches/answer-array
[benchmark-application]: https://github.com/exercism/rust/blob/main/exercises/practice/bob/.articles/performance/code/main.rs
