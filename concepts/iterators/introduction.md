# Introduction

In Rust, any type that implements the [`Iterator`](https://doc.rust-lang.org/stable/std/iter/trait.Iterator.html)
trait can be treated as an iterator.
Rust's built-in [`collections`](https://doc.rust-lang.org/std/collections/index.html)
can be converted into iterators via `iter`, `iter_mut`, and `into_iter` methods.
Iterators have a wide array of [powerful methods](https://doc.rust-lang.org/stable/std/iter/trait.Iterator.html#provided-methods),
such as `collect`, `map`, `filter`, and `fold` for you to perform operations on your data.

```rust
use std::collections::HashMap;

let v = vec![0, 1, 2];

let mut iter = v.iter();

// Call next() to return the next value
assert_eq!(Some(&0), iter.next());
assert_eq!(Some(&1), iter.next());
assert_eq!(Some(&2), iter.next());
// Ahen None once it's over
assert_eq!(None, iter.next());

// Loop over iterators with built-in methods, such as `map` and `collect`
let w: HashMap<_, _> = v.into_iter().map(|x| (x, x + 1)).collect();
println!("{:?}", w);
// => {1: 2, 0: 1, 2: 3}
```