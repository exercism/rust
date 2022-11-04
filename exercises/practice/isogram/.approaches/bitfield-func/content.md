# Bit field used functionally

```rust
const A_LCASE: u8 = 97;

pub fn check_bits_func(candidate: &str) -> bool {
    candidate
        .bytes()
        .filter_map(|c| {
            c.is_ascii_alphabetic()
                .then(|| 1u32 << (c.to_ascii_lowercase() - A_LCASE))
        })
        .try_fold(0u32, |ltr_flags, ltr| {
            (ltr_flags & ltr == 0).then(|| ltr_flags | ltr)
        })
        .is_some()
}
```
