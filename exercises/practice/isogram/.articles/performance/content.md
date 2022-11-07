# Performance

In this approach, we'll find out how to most efficiently determine if a string is an Isogram in Rust.

The [approaches page][approaches] lists three idiomatic approaches to this exercise:

1. [Using `filter` and `map` with `all` on a `HashSet`][approach-filter-all]
2. [Using a bit field in a `for` loop][approach-bitfield]
3. [Using a bit field functionally][approach-bitfield-functionally]

In addition, variants of two of the approaches were benchmarked.

At the time of this writing, all tests use [ASCII][ascii] characters, so the letters can be processed as bytes instead of Unicode characters.

## Benchmarks

To benchmark the approaches, we wrote a [small benchmark application][benchmark-application].

```
test test_check_hash                 ... bench:         952 ns/iter (+/- 176) // using filter, then map
test test_check_bits                 ... bench:          17 ns/iter (+/- 3)
test test_check_bits_func            ... bench:          22 ns/iter (+/- 4)   // using filter_map
test test_check_hash_filtermap       ... bench:         970 ns/iter (+/- 216) // using filter_map
test test_check_bits_func_filter_map ... bench:          37 ns/iter (+/- 7)   // using filter, then map
test test_check_hash_unicode         ... bench:       1,052 ns/iter (+/- 327)
```

The `HashSet` approach was also benchmarked using [`filter_map`][filter-map] instead of `filter` and `map`.

```rust
pub fn check(candidate: &str) -> bool {
    let mut hs = HashSet::new();
    candidate
        .bytes()
        .filter_map(|c| c.is_ascii_alphabetic().then(|| c.to_ascii_lowercase()))
        .all(|c| hs.insert(c))
}
```

The `filter_map` usually benched a bit slower than `filter` and `map`.

The opposite was true for the functional bitfield approach.

It was benchmarked using `filter` and `map` instead of `filter_map`.

```rust
pub fn check(candidate: &str) -> bool {
    candidate
        .bytes()
        .filter(|c| c.is_ascii_alphabetic())
        .map(|c| 1u32 << (c.to_ascii_lowercase() - A_LCASE))
        .try_fold(0u32, |ltr_flags, ltr| {
            (ltr_flags & ltr == 0).then(|| ltr_flags | ltr)
        })
        .is_some()
}
```

The `filter_map` usually benched faster than `filter` and `map`.

All of the approaches are idiomatic.
The two approaches using a bit field are more than thirty-five times faster than the one using a `HashSet`.
Of the two bit field approaches, the `for` loop is a few nanoseconds faster than the one using a chain of functions.
Since they are so close in performance, choosing between them would be a matter of stylistic preference.

[approaches]: https://exercism.org/tracks/rust/exercises/isogram/approaches
[approach-filter-all]: https://exercism.org/tracks/rust/exercises/isogram/approaches/filter-all
[approach-bitfield]: https://exercism.org/tracks/rust/exercises/isogram/approaches/bitfield
[approach-bitfield-functionally]: https://exercism.org/tracks/rust/exercises/isogram/approaches/bitfield-functionally
[benchmark-application]: https://github.com/exercism/rust/blob/main/exercises/practice/isogram/.articles/performance/code/main.rs
[ascii]: https://www.asciitable.com/
[filter-map]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.filter_map
