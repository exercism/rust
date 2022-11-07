# `HashSet` with `is_subset`

```rust
use std::collections::HashSet;

pub fn is_pangram(sentence: &str) -> bool {
    let all: HashSet<char> = HashSet::from_iter("abcdefghijklmnopqrstuvwxyz".chars());
    let used: HashSet<char> = HashSet::from_iter(sentence.to_lowercase().chars());
    all.is_subset(&used)
}
```
