# `HashSet` with `is_subset`

```rust
use std::collections::HashSet;

pub fn is_pangram(sentence: &str) -> bool {
    let all: HashSet<char> = HashSet::from_iter("abcdefghijklmnopqrstuvwxyz".chars());
    let used: HashSet<char> = HashSet::from_iter(sentence.to_lowercase().chars());
    all.is_subset(&used)
}
```

In this approach a [HashSet][hashset] is made of the lowercase alphabet [chars][chars] using the [from_iter][from-iter] method,
and another `HashSet` is made from the [to_lowercase][to-lowercase] `sentence` `chars`.

The function returns if the alphabet `HashSet` [is_subset][is-subset] of the `sentence` `HashSet`.
If all of the letters in the alphabet are a subset of the letters in the `sentence`,
then `is_subset` will return `true`.

[hashset]: https://doc.rust-lang.org/std/collections/struct.HashSet.html
[chars]: https://doc.rust-lang.org/std/primitive.str.html#method.chars
[from-iter]: https://doc.rust-lang.org/std/iter/trait.FromIterator.html#tymethod.from_iter
[to-lowercase]: https://doc.rust-lang.org/std/primitive.str.html#method.to_lowercase
[is-subset]: https://doc.rust-lang.org/std/collections/hash_set/struct.HashSet.html#method.is_subset
