# Introduction

There are a couple of different approaches available to solve Sum of Multiples.
One is to follow the algorithm [outlined in the exercise description][approach-from-factors],
but there are other ways, including [scanning the entire range][approach-from-range]. 

## General guidance

The key to solving Sum of Multiples is to find the unique multiples of all provided factors.
To determine if `f` is a factor of a given number `n`, we can use the [remainder operator][rem].
It is also possible to find the multiples by simple addition, starting from the factor.

## Approach: Calculating sum from factors

```rust
pub fn sum_of_multiples_from_factors(limit: u32, factors: &[u32]) -> u32 {
    factors
        .iter()
        .filter(|&&factor| factor != 0)
        .flat_map(|&factor| (factor..limit).step_by(factor as usize))
        .collect::<HashSet<_>>()
        .iter()
        .sum()
}
```

For more information, check the [Sum from factors approach][approach-from-factors].

## Approach: Calculating sum by iterating the whole range

```rust
pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    (1..limit)
        .filter(|&n| factors.iter().any(|&factor| factor != 0 && n % factor == 0))
        .sum()
}
```

For more information, check the [Sum by iterating the whole range approach][approach-from-range].

## Which approach to use?

- Computing the sum from factors can be efficient if we have a small number of `factors` and/or if they are large compared to the `limit`, because this will result in a small number of hashes to compute "in vain".
  However, as the number of multiples grows, this approach can result in a lot of effort updating the `HashMap` to eliminate duplicates.
- Computing the sum by iterating the whole range can be efficient if we have a small range (low `limit`) and a comparatively large amount of `factors`.
  Additionally, this approach has the advantage of having stable complexity that is only dependent on the limit and the number of factors, since there is no deduplication involved.
  It also avoids any additional memory allocation.

Without proper benchmarks, the second approach may be preferred since it offers a more stable level of complexity (e.g. its performances varies less when the size of the input changes).
However, if you have some knowledge of the size and shape of the input, then benchmarking might reveal that one approach is better than the other for your specific use case.

[approach-from-factors]: https://exercism.org/tracks/rust/exercises/sum-of-multiples/approaches/from-factors
[approach-from-range]: https://exercism.org/tracks/rust/exercises/sum-of-multiples/approaches/from-range
[rem]: https://doc.rust-lang.org/core/ops/trait.Rem.html
