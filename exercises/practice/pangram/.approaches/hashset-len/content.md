# `HashSet` with `len`

```rust
use std::collections::HashSet;

pub fn is_pangram(sentence: &str) -> bool {
    sentence
        .to_lowercase()
        .chars()
        .filter(|c| c.is_ascii_alphabetic())
        .collect::<HashSet<char>>()
        .len()
        == 26
}
```

This approach chains several functions together to determine the result.

- It first passes the `sentence` [to_lowercase][to-lowercase].
- The lowercased `sentence` is then iterated by [chars][chars].
- The `chars` are [filter][filter]ed in its [closure][closure] so that only a character that [is_ascii_alphabetic][is-ascii-alphabetic]
makes it through to be [collect][collect]ed into a [HashSet][hashset].
- The function returns if the [len][len] of the `HashSet` is `26`.
If the number of unique letters in the `HashSet` is equal to the `26` letters in the alphabet, then the function will return `true`.

[to-lowercase]: https://doc.rust-lang.org/std/primitive.str.html#method.to_lowercase
[chars]: https://doc.rust-lang.org/std/primitive.str.html#method.chars
[filter]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.filter
[closure]: https://doc.rust-lang.org/rust-by-example/fn/closures.html
[is-ascii-alphabetic]: https://doc.rust-lang.org/std/primitive.u8.html#method.is_ascii_alphabetic
[collect]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.collect
[hashset]: https://doc.rust-lang.org/std/collections/struct.HashSet.html
[len]: https://doc.rust-lang.org/std/collections/struct.HashSet.html#method.len
