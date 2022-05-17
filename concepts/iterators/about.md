# About

In Rust, any type that implements the [`Iterator`](https://doc.rust-lang.org/stable/std/iter/trait.Iterator.html)
trait can be treated as an iterator.
Rust's built-in [`collections`](https://doc.rust-lang.org/std/collections/index.html),
which include the commonly used `Vec` and `HashMap`, all implement a trait called
[`IntoIterator`](https://doc.rust-lang.org/std/iter/trait.IntoIterator.html).
The `IntoIterator` enables these collections to turn into iterators via `iter`, `iter_mut`, and `into_iter` methods.
The iterators provide a wide array of [powerful methods](https://doc.rust-lang.org/stable/std/iter/trait.Iterator.html#provided-methods),
such as `collect`, `map`, `filter`, and `fold` for you to perform operations on your data.

```rust
let a = vec![0, 1, 2];

let mut iter = a.iter();

// Call next() to return the next value
assert_eq!(Some(&0), iter.next());
assert_eq!(Some(&1), iter.next());
assert_eq!(Some(&2), iter.next());
// Ahen None once it's over
assert_eq!(None, iter.next());

// Loop over iterators with built-in methods, such as `map` and `collect`
let w: Vec<i32> = v.into_iter().map(|x| x + 1).collect();
println!("{:?}", w);
// => [2, 3, 4]
```
