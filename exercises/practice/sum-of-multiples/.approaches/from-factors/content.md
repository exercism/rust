# Calculating sum from factors

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

This approach implements the exact steps outlined in the exercise description:

1. For each non-zero factor, find all multiples of that factor that are less than the `limit`
2. Gather the multiples into a [`BTreeSet`][btreeset] (which automatically removes duplicates)
3. Calculate the sum of all unique multiples

In order to compute the list of multiples for a factor, we create a [`RangeFrom`][rangefrom]
starting with the factor, then use [`step_by`][iterator-step_by] with the same factor.
Because the range has no upper bound, we then use [`take_while`][iterator-take_while] to
stop the iteration when we reach the `limit`.

To combine the multiples of all factors, we iterate the list of factors and use [`flat_map`][iterator-flat_map]
on each factor's multiples. `flat_map` is a combination of [`map`][iterator-map] and [`flatten`][iterator-flatten];
it maps each factor into its multiples, then flattens them all in a single sequence.

To gather the multiples into a [`BTreeSet`][btreeset] which removes duplicates automatically,
we use [`collect`][iterator-collect]. This powerful method can gather the values in an iterator
into many different type of sequences; because of this, the compiler cannot infer the exact output
type we're seeking. To help the compiler, we specify the desired output type using the so-called
[turbofish] syntax: `::<>`. (Note that although we need to specify that the output needs to be
a `BTreeSet`, we do not need to specify the set's _type_ - this can be inferred by the compiler.
To let the compiler infer the set's type, we use the placeholder type specification: `_`.)

Finally, calculating the sum of the remaining unique multiples in the set is easy: we can simply call [`sum`][iterator-sum].

## Nightly Rust and `partition_dedup`

Collecting the multiples in a [`BTreeSet`][btreeset] works, but might be expensive if we insert many multiples in it.
If we're ready to use a Nightly Rust compiler, we can take advantage of a method that is not yet stabilized: [`partition_dedup`][slice-partition_dedup].
This method removes consecutive duplicates from a slice. If the slice was sorted to begin with, then all duplicates are removed.
Furthermore, it works in-place by moving duplicates to the end of the slice, thus not allocating new memory.

If we use [`partition_dedup`][slice-partition_dedup], our code becomes:

```rust
#![feature(slice_partition_dedup)]

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut multiples: Vec<_> = factors
        .iter()
        .filter(|&&factor| factor != 0)
        .flat_map(|&factor| multiples(limit, factor))
        .collect();

    multiples.sort();
    let (unique_multiples, _) = multiples.partition_dedup();
    unique_multiples.iter().sum()
}
```

Here, instead of using a [`BTreeSet`][btreeset], we simply collect the multiples into a [`Vec`][vec]. This is
more efficient since `Vec` will allocate much less memory. Then, in order to have all duplicates removed
by [`partition_dedup`], we first [`sort`][slice-sort] the multiples. (For an explanation of why we use [`sort`][slice-sort]
and not [`sort_unstable`][slice-sort_unstable], see the documentation of [`sort`].)

Running the tests for this variant requires the use of a Nightly Rust toolchain. To install up:

```sh
rustup toolchain install nightly
```

Then, run the tests using:

```sh
cargo +nightly test
```

[btreeset]: https://doc.rust-lang.org/std/collections/struct.BTreeSet.html
[rangefrom]: https://doc.rust-lang.org/std/ops/struct.RangeFrom.html
[iterator-step_by]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.step_by
[iterator-take_while]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.take_while
[iterator-map]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.map
[iterator-flatten]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.flatten
[iterator-collect]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.collect
[turbofish]: https://matematikaadit.github.io/posts/rust-turbofish.html
[iterator-sum]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.sum
[slice-partition_dedup]: https://doc.rust-lang.org/std/primitive.slice.html#method.partition_dedup
[vec]: https://doc.rust-lang.org/std/vec/struct.Vec.html
[slice-sort]: https://doc.rust-lang.org/std/primitive.slice.html?search=vec#method.sort
