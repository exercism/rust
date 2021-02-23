`String` and `&str` are two fundamental ways of working with textual data in Rust.

`String` is an owned, vector of `u8` bytes and mutable.
`&str` is a borrowed reference to a String slice and thus immutable.

You can also create and modify `String`'s using one of their many methods, such as `push`.
```rust
let mut a_string = String::from("hello");
a_string.push('!');
```

You can also create a `String` by converting a `&str`:
```rust
let a_string = "hello".to_string();
```
