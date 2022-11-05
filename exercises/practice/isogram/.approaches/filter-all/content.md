# `filter` and `map` with `all` on a `HashSet`

```rust
use std::collections::HashSet;

pub fn check(candidate: &str) -> bool {
    let mut hs = std::collections::HashSet::new();
    candidate
        .bytes()
        .filter(|c| c.is_ascii_alphabetic())
        .map(|c| c.to_ascii_lowercase())
        .all(|c| hs.insert(c))
}
```

With this approach you will instantiate and update a [`HashSet`][hashset] to keep track of the used letters.

A [`use`][use] declaration allows directly calling `HashSet` instead of calling it with its entire [namespace][namespaces].
Without the `use` declaration, the `HashSet` would be instantiated like so

```rust
let mut hs = std::collections::HashSet::new();
```

After the `HashSet` is instantiated, a series of functions are chained from the `candidate` `&str`.
- Since all of the characters are [ASCII][ascii], they can be iterated with the [`bytes`][bytes] method.
Each byte is iterated as a [`u8`][u8], which is an unsigned 8-bit integer.
- The [`filter`][filter] method [borrows][borrow] each byte as a [reference][reference] to a `u8` (`&u8`).
Inside of its [closure][closure] it tests each byte to see if it [`is_ascii_alphabetic`][is-ascii-alphabetic].
Only bytes which are ASCII letters will survive the `filter` to be passed on to the [`map`][map] method.
- The `map` method calls [`to_ascii_lowercase`][to-ascii-lowercase] on each byte.
- Each lowercased byte is then tested by the [`all`][all] method by using the [`insert`][insert] method of `HashSet`.
`all` will return `true` if every call to `insert` returns true.
If a call to `insert` returns `false` then `all` will "short-circuit" and immediately return `false`.
The `insert` method returns whether the value is _newly_ inserted.
So, for the word `"alpha"`, `insert` will return `true` when the first `'a'` is inserted,
but will return `false` when the second `'a'` is inserted.

## Refactoring

## using the `str` method [to_ascii_lowercase][str-to-ascii-lowercase] and no `map`

You might want to to call the `str` method [to_ascii_lowercase][str-to-ascii-lowercase] and save calling `map`,
like so

```rust
    candidate
        .to_ascii_lowercase()
        .bytes()
        .filter(|c| c.is_ascii_alphabetic())
        .all(|c| hs.insert(c))
```

However, changing the case of all characters in a `str` raised the average benchmark a few nanoseconds.
It is a bit faster to `filter` out non-ASCII letters and to change the case of each byte.
Since the performance is fairly close, either may be prefered. 

### using `filter_map`

Since `filter` and `map` are used, this approach could be refactored using the [`filter_map`][filter-map] method.

```rust
use std::collections::HashSet;

pub fn check(candidate: &str) -> bool {
    let mut hs = HashSet::new();
    candidate
        .bytes()
        .filter_map(|c| c.is_ascii_alphabetic().then(|| c.to_ascii_lowercase()))
        .all(|c| hs.insert(c))
}
```

By chaining the [`then`][then] method to the result of `is_ascii_alphabetic`,
and calling `to_ascii_lowercase` in the closure for `then`,
the `filter map` returns only lowercased ASCII letter bytes.
In benchmarking, this approach was slightly slower, but its style may be prefered.

### supporting Unicode

By substituting the [`chars`][chars] method for the `bytes` method,
and by using the Unicode methods [`is_alphabetic`][is-alphabetic] and [`to_lowercase`][char-to-lowercase],
this approach can support Unicode characters.

```rust
use std::collections::HashSet;

pub fn check(candidate: &str) -> bool {
    let mut hs = std::collections::HashSet::new();
    candidate
        .chars()
        .filter(|c| c.is_alphabetic())
        .map(|c| c.to_lowercase().to_string())
        .all(|c| hs.insert(c))
}
```

Usually an approach that supports Unicode will be slower than one that supports only bytes.
However the benchmark for this approach was significantly slower, taking more than twice as long as the bytes approach.
It can be further refactored to use the `str` [to_lowercase][str-to-lowercase] method
to cut the time down to only slightly slower than the byte approach.

```rust
use std::collections::HashSet;

pub fn check(candidate: &str) -> bool {
    let mut hs = std::collections::HashSet::new();
    candidate
        .to_lowercase()
        .chars()
        .filter(|c| c.is_alphabetic())
        .all(|c| hs.insert(c))
}
```

[hashset]: https://doc.rust-lang.org/std/collections/struct.HashSet.html
[use]: https://doc.rust-lang.org/reference/items/use-declarations.html
[namespaces]: https://doc.rust-lang.org/reference/names/namespaces.html
[ascii]: https://www.asciitable.com/
[bytes]: https://doc.rust-lang.org/std/primitive.str.html#method.bytes
[u8]: https://doc.rust-lang.org/std/primitive.u8.html
[filter]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.filter
[closure]: https://doc.rust-lang.org/rust-by-example/fn/closures.html
[borrow]: https://doc.rust-lang.org/rust-by-example/scope/borrow.html
[reference]: https://doc.rust-lang.org/std/primitive.reference.html
[is-ascii-alphabetic]: https://doc.rust-lang.org/std/primitive.u8.html#method.is_ascii_alphabetic
[map]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.map
[to-ascii-lowercase]: https://doc.rust-lang.org/std/primitive.u8.html#method.to_ascii_lowercase
[all]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.all
[insert]: https://doc.rust-lang.org/std/collections/struct.HashSet.html#method.insert
[str-to-ascii-lowercase]: https://doc.rust-lang.org/std/primitive.str.html#method.to_ascii_lowercase
[filter-map]: https://doc.rust-lang.org/core/iter/trait.Iterator.html#method.filter_map
[then]: https://doc.rust-lang.org/core/primitive.bool.html#method.then
[chars]: https://doc.rust-lang.org/core/primitive.str.html#method.chars
[is-alphabetic]: https://doc.rust-lang.org/core/primitive.char.html#method.is_alphabetic
[char-to-lowercase]: https://doc.rust-lang.org/core/primitive.char.html#method.to_lowercase
[str-to-lowercase]: https://doc.rust-lang.org/std/primitive.str.html#method.to_lowercase
