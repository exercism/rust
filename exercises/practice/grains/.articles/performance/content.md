# Performance

In this approach, we'll find out how to most efficiently calculate the value for Grains in Rust.

The [approaches page][approaches] lists two idiomatic approaches to this exercise:

1. [Using `pow`][approach-pow]
2. [Using bit shifting][approach-bit-shifting]

Varieties of `pow` that iterate and call `square` were also benchmarked.

## Benchmarks

To benchmark the approaches, we wrote a [small benchmark application][benchmark-application].

```
test test_square_bit      ... bench:           0 ns/iter (+/- 0)
test test_total_bit       ... bench:           0 ns/iter (+/- 0)
test test_square_pow      ... bench:           0 ns/iter (+/- 0)
test test_total_pow_u128  ... bench:           0 ns/iter (+/- 0)
test test_total_pow_fold  ... bench:         215 ns/iter (+/- 7)
test test_total_pow_for   ... bench:         263 ns/iter (+/- 46)
```

`pow` and bit shifting have the same measurable performance.
`pow` _may_ be considered to be a bit more readable.

Some community solutions call `square` in a `fold` something like so

```rust
pub fn total() -> u64 {
    (1_u32..=64_u32).fold(0u64, |total, num| total + square_pow(num))
}
```

Although `fold` may be considered a more "functional" approach, it is signficantly slower than the two main approaches.

Some community solutions call `square` in a `for` loop something like so

```rust
pub fn total() -> u64 {
    let mut accum = 0;
    for i in 1..=64 {
        accum += square_pow(i)
    }
    accum
}
```

Using a `for` loop may be considered less idiomatic for this than `fold`, plus it benched somewhat slower than `fold`,
and benched much slower than the two main approaches.

[approaches]: https://exercism.org/tracks/rust/exercises/grains/approaches
[approach-pow]: https://exercism.org/tracks/rust/exercises/grains/approaches/pow
[approach-bit-shifting]: https://exercism.org/rust/csharp/exercises/grains/approaches/bit-shifting
[benchmark-application]: https://github.com/exercism/rust/blob/main/exercises/practice/grains/.articles/performance/code/main.rs
