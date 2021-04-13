# About

[`fold`][fold] is a [consuming iterator adaptor][consuming iterator adaptor] which applies a function to each element of the iteration, accumulating
the result into a new value.

Each iteration of `fold` takes two arguments. The first is an initial value for the accumulator. The second is a closure which itself takes
two arguments: the accumulator and an item. The closure's return value is the subsequent value of the accumulator.
Once the iterator drains, the final value of the accumulator is returned.

`fold` can be used when it is desired to transform a collection into a single value. For example, if the sum of all even numbers is desired

```rust
pub fn main() {
    let even_sum = (1..=10).fold(0, |acc, num| if num % 2 == 0 { acc + num } else { acc });
    println!("{:?}", even_sum);
}
```

Prints `30`.

What can be done in `fold` can often be done by another function or functions. For the above, to [`filter`][filter] on `(1..=10)` and then [`sum`][sum] would
be less verbose and evaluate the same result, like so

```rust
(0..=10).filter(|n| *n % 2 == 0).sum()
```

Other functions that are more commonly used than `fold` include [`product`][product] and [`collect`][collect].

An accumulator can be used for other than numeric values. For example

```rust
pub fn giant_grunts(initial: char) -> String {
    ["Bee", "Fee", "Gee", "Fi", "Hi", "Fo", "Mo", "Fum", "Tum"].iter().fold(
        String::new(),
        |acc, grunt| if grunt.starts_with(initial) { acc + grunt } else { acc },
    )
}

pub fn main() {
    let song = giant_grunts('F');
    println!("{:?}", song);
}
```

Prints `FeeFiFoFum`

[fold]: https://doc.rust-lang.org/beta/std/iter/trait.Iterator.html#method.fold
[consuming iterator adaptor]: https://doc.rust-lang.org/book/ch13-02-iterators.html#methods-that-consume-the-iterator
[filter]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.filter
[sum]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.sum
[product]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.product
[collect]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.collect
