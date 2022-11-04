# Performance

In this approach, we'll find out how to most efficiently determine if a string is an Isogram in Rust.

The [approaches page][approaches] lists three idiomatic approaches to this exercise:

1. [Using `filter` and `map` with `all` on a `HashSet`][approach-filter-all]
2. [Using a bit field in a `for` loop][approach-bitfield]
3. [Using a bit field functionally][approach-bitfield-func]

At the time of this writing, all tests use [ASCII][ascii] characters, so the letters can be processed as bytes instead of Unicode characters.

## Benchmarks

To benchmark the approaches, we wrote a [small benchmark application][benchmark-application].

```
test test_check_hash      ... bench:         774 ns/iter (+/- 39)
test test_check_bits      ... bench:          17 ns/iter (+/- 1)
test test_check_bits_func ... bench:          21 ns/iter (+/- 0)
```
All of the approaches are idiomatic.
The two approaches using a bit field are more than thirty-five times faster than the one using a `HashSet`.
Of the two bit field approaches, the `for` loop is few nanoseconds faster than the one using a chain of functions.
Since they are so close in performance, choosing between them would be a matter of stylistic preference.

[approaches]: https://exercism.org/tracks/rust/exercises/isogram/approaches
[approach-filter-all]: https://exercism.org/tracks/rust/exercises/isogram/approaches/filter-all
[approach-bitfield]: https://exercism.org/tracks/rust/exercises/isogram/approaches/bitfield
[approach-bitfield-func]: https://exercism.org/tracks/rust/exercises/isogram/approaches/bitfield-func
[benchmark-application]: https://github.com/exercism/rust/blob/main/exercises/practice/isogram/.articles/performance/code/main.rs
