The entry API is often the most idiomatic way to manipulate the contents of a map structure. It's especially useful when you need to alter multiple entries in a map structure.

This exercise covered one particular pattern that the entry API is commonly used for, namely using `HashMap`s to store the frequency of elements.

Another way this same pattern can be achieved is by using the `and_modify` method in conjunction with either the `or_default` or `or_insert` methods:

```rust
map.entry(key).and_modify(|v| *v += 1).or_default();
```

```rust
map.entry(key).and_modify(|v| *v += 1).or_insert(0);
```

Other methods on the entry API, such as the `insert` [method](https://doc.rust-lang.org/std/collections/hash_map/enum.Entry.html#method.insert) and the `key` [method](https://doc.rust-lang.org/std/collections/hash_map/enum.Entry.html#method.key), warrant familiarity.
