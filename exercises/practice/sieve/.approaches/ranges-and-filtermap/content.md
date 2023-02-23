# `Range`s and `filter-map()`

```rust
pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    let mut numbers: Vec<_> = (0..=upper_bound).map(Option::from).collect();
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

This approach starts by defining a `Vec` of [`Some`][some] values from `0` through the upper bound.
To minimize type conversions, the upper bound is redefined as a [`usize`][usize].

A [Range][range] is defined that goes from `2` through the length of the `Vec`.
Each number from the range is passed to the [`filter_map()`][filtermap].

The [closure][closure] (also known as a lambda) in the body of the `filter_map()` uses the [`take()`][take] method, combined with the
unwrap operator (`?`), to get the element in the `Vec` at the index of the number passed in from the range.
If the element value is `None`, then no further processing happens in the lambda for that iteration.
If the element value is `Some` number, then an inner range is defined, starting from the number plus itself and going through the upper bound.

The [`step_by()`][stepby] method is used to traverse the range in steps the size of the element value.

Each number from the inner range is passed to the [`for_each()`][foreach] method.
The lambda inside the `for_each` sets `None` as the value for the element in the `Vec` at the index of the value passed in.

After the inner range is done, the `filter_map()` lambda returns the value passed in from the outer range re-wrapped in a `Some`
as type `u64`.

After the outer range is done, all of the `Some` values are [collect][collect]ed from the `Vec` and returned.
[Type inference][type-inference] is used to return the values as a type `Vec<u64>`.

[some]: https://doc.rust-lang.org/std/option/enum.Option.html#variant.Some
[none]: https://doc.rust-lang.org/std/option/enum.Option.html#variant.None
[usize]: https://doc.rust-lang.org/std/primitive.usize.html
[range]: https://doc.rust-lang.org/reference/expressions/range-expr.html
[filtermap]: https://doc.rust-lang.org/core/iter/trait.Iterator.html#method.filter_map
[closure]: https://doc.rust-lang.org/rust-by-example/fn/closures.html
[take]: https://doc.rust-lang.org/core/option/enum.Option.html#method.take
[stepby]: https://doc.rust-lang.org/core/iter/trait.Iterator.html#method.step_by
[foreach]: https://doc.rust-lang.org/core/iter/trait.Iterator.html#method.for_each
[type-inference]: https://doc.rust-lang.org/rust-by-example/types/inference.html
[collect]: https://doc.rust-lang.org/core/iter/trait.Iterator.html#method.collect
