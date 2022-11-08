# Performance

In this approach, we'll find out how to most efficiently determine if a string is a Pangram in Rust.

The [approaches page][approaches] lists four  idiomatic approaches to this exercise:

1. [Using `all` with `contains` on lowercased letters][approach-all-contains]
2. [Using `HashSet` with `is_subset`][approach-hashset-is-subset]
3. [Using `HashSet` with `len`][approach-hashset-len]
4. [Using a bit field to keep track of used letters][approach-bitfield]

## Benchmarks

To benchmark the approaches, we wrote a [small benchmark application][benchmark-application].

```
test test_is_pangram_all_contains   ... bench:         850 ns/iter (+/- 525)
test test_is_pangram_hash_is_subset ... bench:       3,113 ns/iter (+/- 188)
test test_is_pangram_hashset_len    ... bench:       1,737 ns/iter (+/- 331)
test test_is_pangram_bitfield       ... bench:          76 ns/iter (+/- 12)
```

- The `HashSet` `len` approach is not quite twice as fast as the `HashSet` `is_subset` approach.
- The `all` `contains` approach is about twice as fast as the `HashSet` `len`approach.
- The bit field approach is more than ten times as fast as the `all` `contains` approach.
- So, the bit field approach is about forty times faster than the `HashSet` `is_subset` approach.

[benchmark-application]: https://github.com/exercism/rust/blob/main/exercises/practice/pangram/.articles/performance/code/main.rs
[approaches]: https://exercism.org/tracks/rust/exercises/pangram/approaches
[approach-all-contains]: https://exercism.org/tracks/rust/exercises/pangram/approaches/all-contains
[approach-hashset-is-subset]: https://exercism.org/tracks/rust/exercises/pangram/approaches/hashset-is-subset
[approach-hashset-len]: https://exercism.org/tracks/rust/exercises/pangram/approaches/hashset-len
[approach-bitfield]: https://exercism.org/tracks/rust/exercises/pangram/approaches/bitfield
