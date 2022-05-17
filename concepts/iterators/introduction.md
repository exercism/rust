# Introduction

In Rust, an iterator is any value that implements the [`Iterator`](https://doc.rust-lang.org/stable/std/iter/trait.Iterator.html).
The values of an iterator can be manually advanced by calling the `next` method on the iterator.

```rust
trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
    // many built-in methods implemented
}
```

Moreover, if there is a way to iterate over a certain type,
such as `Vec` or `HashMap` of Rust's [`collections`](https://doc.rust-lang.org/std/collections/index.html),
then this type can implement [`IntoIterator`](https://doc.rust-lang.org/std/iter/trait.IntoIterator.html).

```rust
trait IntoIterator where Self::IntoIter: Iterator<Item=Self::Item> {
    type Item;
    type IntoIter: Iterator;
    fn into_iter(self) -> Self::IntoIter;
}
```

An understanding of `traits` is not necessary for this concept and we will have a whole exercise for you to master `traits`.
For now, just remember that:

- Any type that implements `IntoIterator` is called an _iterable_ because it can be turned into an iterator.
- And any type that implements `Iterator` is an iterator, which _produces_ values and can be _iterated over_.
- You can get an iterator from any type that implements `IntoIterator` by calling its `into_iter` method.

## `into_iter` and for loops

As you can see, any type that implements `IntoIterator` has an `into_iter` method.
In fact, you're probably familiar with for loops over a vector (`Vec<T>`):

```rust
// capitals is a `Vec`, which implements `IntoIterator`
let lor = vec!["Ainur", "Elves", "Men", "Dwarves", "Hobbits", "Orcs", "Dragons"];
for people in &lor {
    println!("{}", people);
}
```

which are just a shorthand for calling the vector's `into_iter` method to turn the `iterable` into an `iterator`
and then calls the `next` method on the `iterator` until it produces no more values (i.e. returns `None`):

```rust
// capitals is a `Vec`, which implements `IntoIterator`
let mut peoples = (&lor).into_iter();

// capitals_iterator implements `Iterator`, which has the default `next` method
while let Some(people) = peoples.next() {
    println!("{}", people);
}
```

## `iter` and `iter_mut` methods

Most Rust collection types (such as `Vec`, `HashMap`, `HashSet`) provide `iter` and `iter_mut` methods.
For an iterator that produces values of type `T`, these two methods perform the following by convention:

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

Now that we've covered `iter`, `iter_mut`, and `into_iter`, what are the differences?
Let's amend our previous bullet points:

- The iterator returned by `iter` will yield immutably borrowed `&T`.
- The iterator returned by `iter_mut` will yield mutably borrowed `&mut T`.
- The iterator returned by `into_iter` may yield `T`, `&T` or `&mut T`, depending on its context.

Wait a second, what do we mean by _depending on its context_?
Just what `into_iter` is invoked on.
Let's clarify using the following bullet points:

- `(&v).into_iter()` is equal to `v.iter()` => returns `&T`
- `(&mut v).into_iter()` is equal to `v.iter_mut()` => returns `&mut T`
- `v.into_iter()` => returns `T` and moves the values

This is most clear in for loops:

- `for num in &v {...}` => returns `&T`
- `for num in &mut v {...}` => returns `&mut T`
- `for num in v {...}` => returns `T` and moves the values

Unless we are working with for loops, it's always clearer to write `v.iter()` instead of `(&v).into_iter()`.
Therefore, there are ergonomic reasons for preferring `iter` and `iter_mut` over `into_iter`.

## Using built-in methods for iterators

Iterators provide numerous built-in methods, such as `map`, `filter`, `collect`.
These methods are sometimes used with closures.

Using `map` with a closure to transform a vector into a different one:

```rust
let v = [1, 2, 3];
let v2 iter = v.iter().map(|x| 2 * x);
println!("{:?}", v2);
// => [2, 4, 6]
```

Using `filter` with a closure to filter for certain attributes and then use `collect` to return a collection again:

```rust
let lor = vec!["Ainur", "Elves", "Men", "Dwarves", "Hobbits", "Orcs", "Dragons"];
let short_people = ["Dwarves", "Hobbits"];

let short_filter: Vec<&str> = lor
    .into_iter()
    .filter(|people| short_people.contains(people))
    .collect();

println!("{:?}", short_filter);
// => ["Dwarves", "Hobbits"]
```

## Laziness

It is important to know that Rust iterators are lazy:
they have no effect until we call certain methods that can consume the iterator.
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

To make the iterator not lazy, we must utilize consumer methods, which take an iterator and return something other than an iterator,
thus consuming the iterator in the process.

For instance, `collect` is a consumer method (its function signature does not return an iterator but a generic type):

```rust
//         Iterator   Adaptor        Consumer
//                |      |               |
let v: Vec<_> = (0..5).map(|x| x * x).collect();
```
