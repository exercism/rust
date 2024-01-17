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
use std::collections::BTreeSet;

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    factors
        .iter()
        .filter(|&&factor| factor != 0)
        .flat_map(|&factor| multiples(limit, factor))
        .collect::<BTreeSet<_>>()
        .into_iter()
        .sum()
}

fn multiples(limit: u32, factor: u32) -> impl Iterator<Item = u32> {
    (factor..)
        .step_by(factor as usize)
        .take_while(move |&n| n < limit)
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

- Computing the sum from factors can be efficient if we have a small number of factors and/or if they are large
  compared to the limit, because this will result in a small number of multiples to deduplicate. However, as the
  number of factors increases or if they are small, the number of multiples grows quickly and this approach
  can result in a lot of work to deduplicate them. This approach's complexity is also difficult to pinpoint because
  it is dependent on both the number and the size of the factors, making its performance vary wildly.
- Computing the sum by iterating the whole range is less efficient for large ranges when the number of factors is
  small and/or when they are large. However, this approach has the advantage of having stable complexity that is
  only dependent on the limit and the number of factors. It also avoids any additional memory allocation.

For more information, check the [Performance article][article-performance].

[approach-from-factors]: https://exercism.org/tracks/rust/exercises/sum-of-multiples/approaches/from-factors
[approach-from-range]: https://exercism.org/tracks/rust/exercises/sum-of-multiples/approaches/from-range
[rem]: https://doc.rust-lang.org/core/ops/trait.Rem.html
[article-performance]: https://exercism.org/tracks/rust/exercises/sum-of-multiples/articles/performance
