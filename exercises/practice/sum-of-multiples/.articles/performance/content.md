# Performance

This article compares the different [approaches] that can be used to solve the Sum of Multiples exercise in Rust:

1. [Calculating sum from factors][approach-from-factors]
2. [Calculating sum by iterating the whole range][approach-from-range]

## Benchmarks

To benchmark the approaches, we wrote a [benchmark] using [`criterion`][crate-criterion].
The benchmark compares the two approaches by running two of the tests in the exercise's test suite.
Both tests use a large limit (10,000) but differ in the number and size of factors:

```
much_larger_factors:
    limit = 10_000
    factors = [43, 47]

solutions_using_include_exclude_must_extend_to_cardinality_greater_than_3:
    limit = 10_000
    factors = [2, 3, 5, 7, 11]    
```

Results from running the benchmark (the average time is the middle one):

```
from_factors: much_larger_factors
                        time:   [7.7217 µs 7.7348 µs 7.7479 µs]

from_factors: solutions_using_include_exclude_must_extend_to_cardinality_greater_than_3
                        time:   [214.45 µs 215.77 µs 217.17 µs]

from_range: much_larger_factors
                        time:   [48.398 µs 48.600 µs 48.846 µs]

from_range: solutions_using_include_exclude_must_extend_to_cardinality_greater_than_3
                        time:   [62.479 µs 62.788 µs 63.129 µs]
```

As mentioned on the [approaches] page, computing the sum from the factors is very efficient with
a low number of factors. However, as the number of factors increase (especially when they are small),
performance degrades quickly.

Computing the sum by iterating the whole range, however, is more stable, but is much less efficient
for the "low number of factors" scenario.

Which approach is the best? Although it depends on the test conditions, it seems that the [second approach][approach-from-range]
offers better performance guarantees when the input cannot be controlled in advance.

[approaches]: https://exercism.org/tracks/rust/exercises/sum-of-multiples/approaches
[approach-from-factors]: https://exercism.org/tracks/rust/exercises/sum-of-multiples/approaches/from-factors
[approach-from-range]: https://exercism.org/tracks/rust/exercises/sum-of-multiples/approaches/from-range
[benchmark]: https://github.com/exercism/rust/blob/main/exercises/practice/sum-of-multiples/.articles/performance/code/sum_of_multiples_bench.rs
[crate-criterion]: https://crates.io/crates/criterion
