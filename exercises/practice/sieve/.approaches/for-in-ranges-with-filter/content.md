# `for` in ranges with `filter()`

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

This approach starts by defining a `Vec` of values from `0` through the upper bound.
Since numbers below `2` are not valid primes, the element at index `1` is set to `0`.

Since squares of numbers are processed, the [`sqrt()`][sqrt] method is used to determine when iterating the `Vec` indexes will stop.

To minimize type conversions, the upper bound is redefined as a [`usize`][usize].

A [`for` in range][for-in-range] is defined from `2` up to but not including the `stop` value.
Each number in the range is used inside the loop as an index to test if the element value in the `Vec` at that index is not `0`.

If the element value is not `0`, then an inner `for` in range is defined, starting with the number times itself and going through the upper bound.
The [`step_by()`][stepby] method is used to traverse the range in steps the size of the outer range number.

Each number from the inner range is used as an index to set the element value at that index in the `Vec` to `0`.

When the outer range is done, the `Vec` is passed through the [`iter()`][iter] method to the [`filter()`][filter] method.
The [closure][closure] (also known as a lambda) in the body of the `filter()` tests that the element is not `0`.
It dereferences the number to convert it from a "reference to a reference" to a "reference", and it references the `0` literal
`u64` value to enable comparison between references.

The surviving values are chained to the [`copied()`][copied] method, which changes the iterator of references to an iterator of the copied values.

The copied values ae chained to the [`collect()`][collect] method, which uses [type inference][type-inference] to return the `Vec<u64>`.

[sqrt]: https://doc.rust-lang.org/std/primitive.f64.html#method.sqrt
[usize]: https://doc.rust-lang.org/std/primitive.usize.html
[for-in-range]: https://doc.rust-lang.org/rust-by-example/flow_control/for.html
[filter]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.filter
[stepby]: https://doc.rust-lang.org/core/iter/trait.Iterator.html#method.step_by
[iter]: https://doc.rust-lang.org/core/primitive.slice.html#method.iter
[filter]: https://doc.rust-lang.org/core/iter/trait.Iterator.html#method.filter
[closure]: https://doc.rust-lang.org/rust-by-example/fn/closures.html
[collect]: https://doc.rust-lang.org/core/iter/trait.Iterator.html#method.collect
[copied]: https://doc.rust-lang.org/core/iter/trait.Iterator.html#method.copied
[type-inference]: https://doc.rust-lang.org/rust-by-example/types/inference.html
