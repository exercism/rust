# About

To understand the `collect` function it's important to know what iterators are. To implement the `Iterator` trait one only has to implement
a `next` method, which has the following signature

```rust
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}
```

As the `next` method implies, implementing the `Iterator` trait generates a sequence of items, usually from a collection such as a vector,
array or a string (which is a collection of chars.) Calling `next` for the first time will yield the first item in the sequence if there is
one, or, if there is nothing in the collection remaining to sequence it will return `None`. However, it is not usual to call the `next` method
directly. The power of iterators is to use [iterator adaptors][iterator adaptors] which traverse an entire sequence to provide some kind of
transformation of the collection's items into another iterator. Common iterator adaptors include: [filter][filter], which retains only items from
a collection that satisfy a condition; [map][map], which transforms the type of the items into another type; and [rev][rev], which reverses the
order of the items. Since iterator adaptors yield another iterator, they can be chained together. For instance, to reverse the characters in a
string one might want an iterator of characters and then a reverse iterator, like so

```rust
let rev_chars = "Hello, World!".chars().rev();
```

But `rev_chars` does not give us a `String` with the characters of "Hello, World!" reversed. It gives us an `impl Iterator<Item = char>`. It's a
type which is _prepared_ to yield a sequence of characters, but it doesn't know how to finally assemble them. In this case it doesn't know if we
want a `Vec<char>` or a `String`.

Even though iterator adaptors do so much, they are _lazy_. But this only means that they are like an arrow drawn in a bow. They are not released
until they have a target, called a [consuming adaptor][consuming adaptor], which uses up either a single iterator or the last iterator in a chain
of iterators.

[`collect`][collect] is one such consuming adaptor, and is what can be used to tell `rev_chars` what type of collection we want to end up with. If
we want a `Vec<char>` we could insert the type into our call to `collect` like so

```rust
    let rev_chars = "Hello, World!".chars().rev().collect::<Vec<char>>();
// since the item type of the `String::chars()` iterator can be inferred to be `char`, we can omit that information
// let rev_chars = "Hello, World!".chars().rev().collect::<Vec<_>>();
```

The double colons and outer angle brackets (i.e. `::<_>`) are known as the `turbofish`.
In this case the turbofish is noisier syntax than we need. By specifying the type on the binding itself the `collect` can infer what
collection it should create with less syntactical noise.

```rust
    let rev_chars: Vec<char> = "Hello, World!".chars().rev().collect();
// or
// let rev_chars: Vec<_> = "Hello, World!".chars().rev().collect();
```

If we wanted a `String` instead we could do it like so

```rust
    let rev_chars: String = "Hello, World!".chars().rev().collect();
```

Rust's type inference is powerful enough that sometimes we don't need an explicit inline note:

```rust
fn get_reversed_chars(phrase: &str) -> Vec<char> {
    phrase.chars().rev().collect()
}
```

The return type of the `get_reversed_chars` function lets the `collect` function infer what collection to return. Here, though, we must specify the
`Vec<char>` and not `Vec<_>` or we get the compilation error

```rust
// the type placeholder `_` is not allowed within types on item signatures
// not allowed in type signatures
// replace with the correct return type
```

Although you can sometimes use `collect` without specifying the type, at other times the compiler will state it requires the type for a `collect`, so it
can be a good habit to be aware of how to specify what type you want to collect.

[iterator adaptors]: https://doc.rust-lang.org/book/ch13-02-iterators.html?highlight=lazy#methods-that-produce-other-iterators
[filter]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.filter
[map]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.map
[rev]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.rev
[consuming adaptor]: https://doc.rust-lang.org/book/ch13-02-iterators.html?highlight=lazy#methods-that-consume-the-iterator
[collect]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.collect
