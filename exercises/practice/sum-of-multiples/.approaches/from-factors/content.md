# Calculating sum from factors

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

This approach implements the exact steps outlined in the exercise description:

1. For each non-zero factor, find all multiples of that factor that are less than the `limit`
2. Collect all multiples in a [`HashSet`][hash_set]
3. Calculate the sum of all unique multiples

In order to compute the list of multiples for a factor, we create a [`Range`][range] from the factor (inclusive) to the `limit` (exclusive), then use [`step_by`][iterator-step_by] with the same factor.

To combine the multiples of all factors, we iterate the list of factors and use [`flat_map`][iterator-flat_map] on each factor's multiples.
[`flat_map`][iterator-flat_map] is a combination of [`map`][iterator-map] and [`flatten`][iterator-flatten]; it maps each factor into its multiples, then flattens them all in a single sequence.

Since we need to have unique multiples to compute the proper sum, we [`collect`][iterator-collect] the multiples into a [`HashSet`][hash_set], which only keeps one of each of its entries, thus removing duplicates.
[`collect`][iterator-collect] is a powerful function that can collect the data in a sequence and store it in any kind of collection - however, because of this, the compiler in this case is not able to infer the type of collection you want as the output.
To solve this problem, we specify the type `HashSet<_>` explicitly.

Finally, calculating the sum of the remaining unique multiples in the set is easy: we can simply get an [Iterator][iterator] and call [`sum`][iterator-sum].

[vec]: https://doc.rust-lang.org/std/vec/struct.Vec.html
[hash_set]: https://doc.rust-lang.org/std/collections/struct.HashSet.html
[range]: https://doc.rust-lang.org/std/ops/struct.Range.html
[iterator]: https://doc.rust-lang.org/std/iter/trait.Iterator.html
[iterator-step_by]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.step_by
[iterator-flat_map]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.flat_map
[iterator-map]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.map
[iterator-flatten]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.flatten
[iterator-collect]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.collect
[slice-sort]: https://doc.rust-lang.org/std/primitive.slice.html#method.sort
[vec-dedup]: https://doc.rust-lang.org/std/vec/struct.Vec.html#method.dedup
[iterator-sum]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.sum
[slice-sort_unstable]: https://doc.rust-lang.org/std/primitive.slice.html#method.sort_unstable
