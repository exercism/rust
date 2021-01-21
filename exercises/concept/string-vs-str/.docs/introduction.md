`String` is a potentially-mutable utf8-encoded representation of a sequence of Unicode codepoints. (In Java or C#, this type is called `StringBuilder`.)

`&str` is a read only view of a well-formed utf8 sequence and because it's a reference it is `Copy` and can be shared.

`String` and `&str` in rust can not be indexed into like you might in other languages.
For example this will not work:

```rust,invalid
let hello = "Hello";
println!("First letter = {}", hello[0]);
```

This derives from the following properties:

- Strings are utf8, whose primitive is not a byte, but a Unicode codepoint: `char`.
- Unicode codepoints are up to 4 bytes wide, but encode with variable width.
- It is therefore impossible to compute the position of the `n`th codepoint without iterating over the actual encoded characters, because past the first character, the position of the next character depends on how wide the previous ones were.
- Rust's standard library authors therefore chose not to implement the `Index` trait for strings: while it takes a constant time to index into a byte array, it might take quite a long time to index into a string. This violates the principle of least surprise.

```rust
let example = "I'm not entirely ascii! ❤";

let n_chars = example.chars().count();
let final_byte = example.as_bytes()[example.len()-1];

println!("n_chars = {}", n_chars);             // 25
println!("example.len() = {}", example.len()); // 27

println!("final char: {}", example.chars().nth(n_chars-1).unwrap()); // ❤
println!("final byte: {}", final_byte);                              // 164
println!("char of final byte: {}", final_byte as char);              // ¤
```

# Converting a `String` into a `&str`

`String` implements `Deref<Target=str>`. This trait has some compiler special casing: it means that any reference to a `String` can automatically, transparently be coerced into an `&str`. This implies the following:

- Any code which expects, for example, an `&str` argument can accept an `&String` instead.
- Any method implemented for `&str` can be called on a `String` as well.

(If one generically wanted to be able to take any form of reference to a `str` then one could have an argument of type `impl AsRef<str>` but typically typing arguments as `&str` is common.)

# Converting a `&str` to a `String`

There are many ways to create a `String` from a `&str`:

- `String::from("my str")`
- `"my str".to_string()`
- `"my str".to_owned()`
- `let s: String = "my str".into();`

All of the above ways require allocating memory for the `String` (on the heap).
This is why `&str` is available in no_std but `String` is not.

(The [heapless](https://crates.io/crates/heapless) crate is often used when manipulating
strings in no_std.)
