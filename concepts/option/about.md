The `Option` type is an enum provided by the standard library. It is useful for expressing a value which may not exist.
While this may sound like `null` or `nil`, it is quite different and special.
`Option` is distinctive to Rust. It is also quite fun to work with!

`Option<T>` holds other types and has two variants `Some` and `None`.

For example, `Option<u32>` holds the `u32` type.

The below snippets demonstrate what this would look like in some Rust code.

When a value exists in an `Option<u32>`:
```rust
let has_something: Option<u32> = Some(100);
assert_eq!(has_something.unwrap(), 100);
```

When a value *does not* exist in an `Option<u32>`:
```rust
let has_nothing: Option<u32> = None;
assert_eq!(has_nothing, None);
```
