# Entry API

The entry API is often the most idiomatic way to manipulate the contents of a map structure. However, unusually for Rust, it hasn't been formalized into a trait; it's just a set of types and methods which are implemented on the standard library's map types.

Fundamentally, the `map.entry()` method provides an `enum`:

```rust
enum Entry {
    Occupied(_),
    Vacant(_),
}
```

The utility comes from further methods implemented on that enum. For example, if you want to get and potentially mutate the value for a particular key, and the value's type default is appropriate, you can just do:

```rust
// `entry` has type `&mut Value` for the value type of the map
let entry = map.entry(key).or_default();
```

For applications like a counter, this makes life very easy:

```rust
let counts: HashMap<_, usize> = HashMap::new();
for item in sequence {
    *counts.entry(item).or_default() += 1;
}
```

In case the type's default value is not suitable for your application, you can provide your own constant default with `or_insert`, or calculate one with `or_insert_with`.

In certain cases, you may wish to perform a calculation on an existing value before any potential inserts. In those cases, `and_modify` permits this. However, this application is somewhat rare. `and_modify` is more commonly used to avoid the need to dereference:

```rust
map.entry(key).and_modify(|v| *v += 1).or_default();
```

Occasionally, it's desirable to match the entry directly. This is permissable:

```rust
match map.entry(key) {
    Entry::Occupied(occupied) => println!("removed from the map: {:?}", occupied.remove_entry()),
    Entry::Vacant(vacant) => println!("key not present in map: {:?}", vacant.key()),
}
```

For more information, see [`std::collections::hash_map::Entry`](https://doc.rust-lang.org/std/collections/hash_map/enum.Entry.html) and [`std::collections::btree_map::Entry`](https://doc.rust-lang.org/std/collections/btree_map/enum.Entry.html).
