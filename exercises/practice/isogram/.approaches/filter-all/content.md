# `filter` with `all` on a `HashSet`

```rust
pub fn check_hash(candidate: &str) -> bool {
    let mut hs = HashSet::new();
    candidate
        .to_lowercase()
        .bytes()
        .filter(|&c| c.is_ascii_alphabetic())
        .all(|c| hs.insert(c))
}
```