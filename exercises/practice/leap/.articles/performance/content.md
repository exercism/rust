# Performance

In this approach, we'll find out how to most efficiently calculate if a year is a leap year in Rust.

The [approaches page][approaches] lists three idiomatic approaches to this exercise:

1. [Using the Boolean one-line approach][approach-boolean-line]
2. [Using the Ternary expression approach][approach-ternary-expression]
3. [Using the `match` on a `tuple` approach][approach-match-on-a-tuple]

For our performance investigation, we'll also include two other approaches:

- [The `time::Date` addition approach][approach-date-add-time]
- [`chrono::Date` addition approach][approach-date-add-chrono]

## Benchmarks

To benchmark the approaches, we wrote a [small benchmark application][benchmark-application].

```
test test_ternary  ... bench:           0 ns/iter (+/- 0)
test test_one_line ... bench:           0 ns/iter (+/- 0)
test test_match    ... bench:           0 ns/iter (+/- 0)
test test_time     ... bench:          30 ns/iter (+/- 9)
test test_chrono   ... bench:          18 ns/iter (+/- 3)
test test_naive    ... bench:          18 ns/iter (+/- 1)
```

The three main approaches were identical in measurable performance.
Of the two date addition approaches, the `chrono` crate delivered a solution that was 40% faster than the `time` crate.

[approaches]: https://exercism.org/tracks/rust/exercises/leap/approaches
[approach-boolean-line]: https://exercism.org/tracks/rust/exercises/leap/approaches/boolean-one-line
[approach-ternary-expression]: https://exercism.org/tracks/rust/exercises/leap/approaches/ternary-expression
[approach-match-on-a-tuple]: https://exercism.org/tracks/rust/exercises/leap/approaches/match-on-a-tuple
[approach-date-add-time]: https://exercism.org/tracks/rust/exercises/leap/approaches/date-addition-time
[approach-date-add-chrono]: https://exercism.org/tracks/rust/exercises/leap/approaches/date-addition-chrono
[benchmark-application]: https://github.com/exercism/rust/blob/main/exercises/practice/leap/.articles/performance/code/main.rs

