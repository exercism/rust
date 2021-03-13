# Hints

## General

- Upon fetching an entry using the `entry` method, the entry can be modified in-place after dereferencing it.

- The `or_insert` [method](https://doc.rust-lang.org/std/collections/hash_map/enum.Entry.html#method.or_insert) inserts the given value in the case when the entry is vacant, and returns a mutable reference to the value in the entry.

```rust
*counter.entry(key).or_insert(0) += 1;
```

- The `or_default` [method](https://doc.rust-lang.org/std/collections/hash_map/enum.Entry.html#method.or_default) ensures a value is in the entry by inserting the default value if empty, and returns a mutable reference to the value in the entry.

```rust
*counter.entry(key).or_default() += 1;
```
