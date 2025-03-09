# Performance

In this approach, we'll find out how to most efficiently determine the response for Bob in Rust.

The [approaches page][approaches] lists two idiomatic approaches to this exercise:

1. [Using `if` statements][approach-if].
2. [Using `match` on a `tuple`][approach-match].

For our performance investigation, we'll also include a third approach that [uses an answer array][approach-answer-array] and
a fourth that uses a state machine.

## Benchmarks

To benchmark the approaches, we wrote a [small benchmark application][benchmark-application].

```
test multiple_line_question_array         ... bench:         175.80 ns/iter (+/- 5.91)
test multiple_line_question_if            ... bench:         157.26 ns/iter (+/- 8.69)
test multiple_line_question_match         ... bench:         152.95 ns/iter (+/- 5.79)
test multiple_line_question_state_machine ... bench:          89.17 ns/iter (+/- 9.65)
```

All four approaches are close in performance, but the `if` statements and `match` approaches may be considered to be more idiomatic. The state machine approach is the fastest because it loops through the characters of the message only once.

[approaches]: https://exercism.org/tracks/rust/exercises/bob/approaches
[approach-if]: https://exercism.org/tracks/rust/exercises/bob/approaches/if-statements
[approach-match]: https://exercism.org/tracks/rust/exercises/bob/approaches/match-on-tuple
[approach-answer-array]: https://exercism.org/tracks/rust/exercises/bob/approaches/answer-array
[benchmark-application]: https://github.com/exercism/rust/blob/main/exercises/practice/bob/.articles/performance/code/main.rs
