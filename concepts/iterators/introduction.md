# Introduction

In Rust, an iterator is any value that implements the [`Iterator`](https://doc.rust-lang.org/stable/std/iter/trait.Iterator.html) trait.
The values of an iterator can be manually advanced by calling the `next` method on the iterator.

```rust
trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
    // many built-in methods implemented
}
```

Many of Rust's [`collections`](https://doc.rust-lang.org/std/collections/index.html) implement
[`IntoIterator`](https://doc.rust-lang.org/std/iter/trait.IntoIterator.html),
which can produce an `Iterator` from the collection's values.

```rust
trait IntoIterator where Self::IntoIter: Iterator<Item=Self::Item> {
    type Item;
    type IntoIter: Iterator;
    fn into_iter(self) -> Self::IntoIter;
}
```

Note that while `IntoIterator::into_iter` consumes its `self` type, that type does not necessarily have to be an owned value.
Thus, there are `IntoIterator` implementations for [`Vec`](https://doc.rust-lang.org/std/vec/struct.Vec.html#impl-IntoIterator-2),
for [`&'a Vec`](https://doc.rust-lang.org/std/vec/struct.Vec.html#impl-IntoIterator),
and for [`&'a mut Vec`](https://doc.rust-lang.org/std/vec/struct.Vec.html#impl-IntoIterator-1).
These implementations for varying degrees of ownership provide useful flexibility.

An understanding of `traits` is not necessary for this concept and we will have a whole exercise for you to master `traits`.
For now, just remember that:

- Any type that implements `IntoIterator` is called an _iterable_ because it can be turned into an iterator.
- Any type that implements `Iterator` is an iterator, which _produces_ values and can be _iterated over_.
- You can get an iterator from any type that implements `IntoIterator` by calling its `into_iter` method.
- Every iterator [implements `IntoIterator`](https://doc.rust-lang.org/std/iter/trait.IntoIterator.html#impl-IntoIterator-25) and produces itself.
- `for` loops always operate on iterators and call `IntoIterator` implicitly on their range argument.

## `into_iter` and for loops

You're probably familiar with for loops over a vector:

```rust
// lor is a `Vec`, which implements `IntoIterator`
let lor = vec!["Ainur", "Elves", "Men", "Dwarves", "Hobbits", "Orcs", "Dragons"];
for people in &lor {
    println!("{people}");
}
```

Fundamentally, a `for` loop is just shorthand for calling its argument's `into_iter` method,
and then calling `next()` until it stops producing items. In other words, the previous loop desugars as:

```rust
{
    let mut iterator = (&lor).into_iter();
    while let Some(people) = iterator.next() {
        println!("{people}");
    }
}

## `iter` and `iter_mut` methods

Most Rust collection types provide `iter` and `iter_mut` methods.
For a collection of type `T`, these two methods perform the following by convention:

- The iterator returned by `iter` will yield immutably borrowed `&T`.
- The iterator returned by `iter_mut` will yield mutably borrowed `&mut T`.

```rust
let mut v = vec![1, 2, 3];

// Using `iter` method
let mut iterator = v.iter();
assert_eq!(iterator.next(), Some(&1));
assert_eq!(iterator.next(), Some(&2));
assert_eq!(iterator.next(), Some(&3));
assert_eq!(iterator.next(), None);

// Using `iter_mut` method
let mut muterator = v.iter_mut();
assert_eq!(muterator.next(), Some(&mut 1));
assert_eq!(muterator.next(), Some(&mut 2));
assert_eq!(muterator.next(), Some(&mut 3));
assert_eq!(muterator.next(), None);
```

## Difference between `iter`, `iter_mut`, and `into_iter`

If `iter`, `iter_mut`, and `into_iter` all produce iterators, what are the differences?

- The iterator returned by `iter` will conventionally yield immutably borrowed `&T`.
- The iterator returned by `iter_mut` will conventionally yield mutably borrowed `&mut T`.
- The iterator returned by `into_iter` may yield `T`, `&T` or `&mut T`, depending on its context.

What is the context for an `into_iter` invocation?
Its `self` type.
Recall that there are `IntoIterator` implementations for `Vec`, `&'a Vec`, and `&'a mut Vec`.
Therefore:

- `(&v).into_iter()` is equal to `v.iter()` => returns `&T`
- `(&mut v).into_iter()` is equal to `v.iter_mut()` => returns `&mut T`
- `v.into_iter()` => returns `T` and moves the values

This is most clear in for loops:

- `for num in &v {...}` => binds `&T`
- `for num in &mut v {...}` => binds `&mut T`
- `for num in v {...}` => binds `T` and moves the values

It is often clearer to write `v.iter()` instead of `(&v).into_iter()`.
Therefore, there are ergonomic reasons for preferring `iter` and `iter_mut` over `into_iter`.

## Using built-in methods for iterators

Iterators provide numerous built-in methods, such as `map`, `filter`, `collect`.
These methods are sometimes used with closures.

`map` uses a closure to transform the values of an iterator.
`take` truncates a (potentially infinite) iterator into one which terminates after at most the specified number of items.

```rust
for even_number in (0..).into_iter().map(|x| x * 2).take(4) {
    let odd_number = even_number + 1;
    println!("{even_number} => {odd_number}");
}
// 0 => 1
// 2 => 3
// 4 => 5
// 6 => 7
```

`filter` transforms the iterator into one which only returns those values for which the closure evaluates to `true`.
`count` consumes the iterator, returning the number of items produced.

```rust
let lor = vec!["Ainur", "Elves", "Men", "Dwarves", "Hobbits", "Orcs", "Dragons"];
let short_people = ["Dwarves", "Hobbits"];
let num_tall_people = lor.iter().filter(|people| !short_people.contains(people)).count();
assert_eq!(num_tall_people, 5);
```

`flat_map` transforms an iterator by transforming each item into a new iterator via the provided closure,
then flattening all the produced new iterators by chaining them together.
`collect` produces a new collection,
as long as the collection type implements [`FromIterator`](https://doc.rust-lang.org/std/iter/trait.FromIterator.html).

```rust
let words = ["alpha", "beta", "gamma"];
// chars() returns an iterator
let merged: String = words.iter()
                          .flat_map(|s| s.chars())
                          .collect();
assert_eq!(merged, "alphabetagamma");
```

The [`Iterator` trait](https://doc.rust-lang.org/std/iter/trait.Iterator.html) provides _many_ such useful iterator transforms;
it's worth reading the documentation and familiarizing yourself with them.

## Laziness

It is important to know that Rust iterators are lazy:
they produce no values until we consume the iterator.
If you write and run the following code:

```rust
(0..5).map(|x| x * x);
```

You will get the following compiler error:

```rust
 --> src/main.rs:2:5
  |
2 |     (0..5).map(|x| x * x);
  |     ^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: `#[warn(unused_must_use)]` on by default
  = note: iterators are lazy and do nothing unless consumed
```

This is because `map` is an adapter method that returns another iterator.

To consume an iterator, either use a `for` loop or use an iterator method which produces some value which is not itself an iterator.

For instance, `collect` is a consumer method (its function signature does not return an iterator but a generic type):

```rust
//         Iterator   Adaptor        Consumer
//                |      |               |
let v: Vec<_> = (0..5).map(|x| x * x).collect();
```
