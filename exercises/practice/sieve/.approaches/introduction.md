# Introduction

There are many ways to implement Sieve.
One approach can use a [`Range`][range] to iterate the indexes of the `Vec`, passing each index to the [`filter_map()`][filtermap] method,
which processes the numbers.
Another approach could use a couple of [for in range][for-in-range] loops to process the numbers, and then use the [`filter()`][filter] method
to return the `Vec`.

## General guidance

One of the things to try to minimize is mutability.
If using `mut` for anything than the `Vec`, then the solution may have more mutability than required.

## Approach: `Range` and `filter-map()`

```rust
pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    let mut numbers: Vec<_> = (0..=upper_bound).map(Option::from).collect();
    numbers[1] = None;
    let upper_bound = upper_bound as usize;
    (2..numbers.len())
        .filter_map(|i| {
            let prime = numbers[i].take()? as usize;
            (prime + prime..=upper_bound)
                .step_by(prime)
                .for_each(|j| numbers[j] = None);
            Some(prime as u64)
        })
        .collect()
}
```

For more information, check the [`Range` and `filter_map()` approach][approach-range-and-filtermap].


## Approach: `for` in range with `filter()`

```rust
pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    let mut work: Vec<u64> = (0..=upper_bound).collect();
    work[1] = 0;
    let stop = (upper_bound as f64).sqrt() as usize + 1usize;
    let upper_bound = upper_bound as usize;
    for i in 2..stop {
        if (work[i]) != 0 {
            for idx in (i * i..=upper_bound).step_by(i) {
                work[idx] = 0;
            }
        }
    }
    work.iter().filter(|num| *num != &0u64).copied().collect()
}
```

For more information, check the [`for` in range with `filter()` approach][approach-for-in-range-with-filter].

## Which approach to use?

The `filter_map` approach may be considered a bit more idiomatic.
Using the following benchmark, the `for` loops approach averaged a bit faster,
but the `filter_map()` approach was more consistent.

```rust
#[bench]
fn limit_of_1000(b: &mut Bencher) {
    b.iter(|| primes_up_to(1000));
}
```

Results

```
// for in range
test limit_of_1000 ... bench:       5,678 ns/iter (+/- 773)

// filter_map
test limit_of_1000 ... bench:       6,448 ns/iter (+/- 430)
```

[range]: https://doc.rust-lang.org/reference/expressions/range-expr.html
[filtermap]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.filter_map
[for-in-range]: https://doc.rust-lang.org/rust-by-example/flow_control/for.html
[filter]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.filter
[mutability]: https://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/book/first-edition/mutability.html
[approach-range-and-filtermap]: https://exercism.org/tracks/rust/exercises/sieve/approaches/range-and-filtermap
[approach-for-in-range-with-filter]: https://exercism.org/tracks/rust/exercises/sieve/approaches/for-in-range-with-filter
