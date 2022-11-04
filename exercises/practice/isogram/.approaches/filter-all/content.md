# `filter` with `all` on a `HashSet`

```rust
pub fn check(candidate: &str) -> bool {
    let mut hs = HashSet::new();
    candidate
        .to_lowercase()
        .chars()
        .filter(|&c| c.is_alphabetic())
        .all(|c| hs.insert(c))
}
```
