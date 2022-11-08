# `all` with `contains` on lowercased letters

```rust
pub fn is_pangram(sentence: &str) -> bool {
    let sentence_lowered = sentence.to_lowercase();
    ('a'..='z').all(|ltr| sentence_lowered.contains(ltr))
}
```

- This begins by lowercasing the input by using [to_lowercase][tolower].
- It then checks if all letters in the alphabet are contained in the `sentence`,
using the [`Iterator`][iterator] method [`all`][all] with the [`str`][str] method [`contains`][contains].
If all of the letters in the alphabet are contained in the `sentence`, then the function will return `true`.

[tolower]: https://doc.rust-lang.org/std/string/struct.String.html#method.to_lowercase
[iterator]: https://doc.rust-lang.org/std/iter/trait.Iterator.html
[all]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.all
[str]: https://doc.rust-lang.org/std/primitive.str.html
[contains]: https://doc.rust-lang.org/std/primitive.str.html#method.contains
