## General

- Upon fetching an entry using the `entry` method, the entry can be modified in-place after dereferencing it.
- The `or_insert` [method](https://doc.rust-lang.org/std/collections/hash_map/enum.Entry.html#method.or_insert) inserts the given value in the case when the entry is vacant.

```rust
*counter.entry(key).or_default() += 1;
```
