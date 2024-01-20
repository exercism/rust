# Calculating sum by iterating the whole range

```rust
pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    (1..limit)
        .filter(|&n| factors.iter().any(|&factor| factor != 0 && n % factor == 0))
        .sum()
}
```

Instead of implementing the steps in the exercise description, this approach uses another angle:

1. Iterate all numbers between 1 (inclusive) and `limit` (exclusive)
2. Keep only numbers which have at least one factor in `factors` (automatically avoiding any duplicates)
3. Calculate the sum of all numbers kept

After creating our range, we use [`filter`][iterator-filter] to keep only matching multiples.
To detect the multiples, we scan the `factors` and use [`any`][iterator-any] to check if at least one is a factor of the number we're checking.
([`any`][iterator-any] is short-circuiting: it stops as soon as it finds one compatible factor.)

Finally, once we have the multiples, we can compute the sum easily using [`sum`][iterator-sum].

[iterator-filter]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.filter
[iterator-any]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.any
[iterator-sum]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.sum
