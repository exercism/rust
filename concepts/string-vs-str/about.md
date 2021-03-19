# About

`String` and `&str` are two fundamental ways of working with textual data in Rust.
Knowing their characteristics and how they work together comes in very handy.
This document discusses how they overlap and offers some basic advice.
`String` is an owned, vector of `u8` bytes.
`String` is mutable.
`&str`, is a borrowed reference to a String slice.
`&str` is immutable.

The proper pronunciation of `&str` is "ref string".
`String` is a potentially-mutable UTF-8-encoded representation of a sequence of Unicode code points.

`&str` is a read only view of a well-formed UTF-8 sequence.
Because it's a reference, it is `Copy` and can be shared.

`String` and `&str` in Rust can not be indexed into like you might in other languages.
For example this will not work:

```rust,invalid
let hello = "Hello";
println!("First letter = {}", hello[0]);
```

This derives from the following properties:

- Strings are UTF-8, whose primitive is not a byte, but a Unicode code point: `char`.
- Unicode code points are up to 4 bytes wide, but encode with variable width.
- It is therefore impossible to compute the position of the `n`th code point without iterating over the actual encoded characters, because past the first character, the position of the next character depends on how wide the previous ones were.
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

## Converting a `String` into a `&str`

`String` implements `Deref<Target=str>`.
This trait has some compiler special casing: it means that any reference to a `String` can automatically, transparently be coerced into an `&str`.
This implies the following:

- Any code which expects, for example, an `&str` argument can accept an `&String` instead.
- Any method implemented for `&str` can be called on a `String` as well.

(If one generically wanted to be able to take any form of reference to a `str` then one could have an argument of type `impl AsRef<str>` but typically typing arguments as `&str` is common.)

## Converting a `&str` to a `String`

There are many ways to create a `String` from a `&str`:

- `String::from("my str")`
- `"my str".to_string()`
- `"my str".to_owned()`
- `let s: String = "my str".into();`

All of the above ways require allocating memory for the `String` (on the heap).
This is why `&str` is available in no_std but `String` is not.

(The [heapless](https://crates.io/crates/heapless) crate is often used when manipulating
strings in no_std.)

## Using String and `&str` in structs

This section discusses when to use `String` or `&str` for struct fields.
You'll often see structs defined with `String` instead of `&str` for field values.

```rust
struct Record {
    latitude: f64,
    longitude: f64,
    population: Option<u64>,
    city: String,
    state: String,
}
```

By using `String`, the struct owns the data.
This makes the code easier to read and teach for beginners.
But it also means heap allocations will occur for each field.
`&str` borrows textual data so will perform better at the trade-off of your struct needing to define lifetime parameters.
The [csv crate tutorial][csv-tutorial] practically demonstrates and explains these ideas quite nicely.

## Performance Notes

`String`'s are allocated on the heap so can be more costly to use.
`&str`'s are a shared reference and so do not require frequent reallocation.

## CSV Specification

CSV is a common format for textual data.
Remember how we said a formal specification exists?
[RFC 4180][rfc-4180] has all your details.
Enjoy!

[csv-tutorial]: https://docs.rs/csv/1.1.5/csv/tutorial/index.html
[rfc-4180]: https://docs.rs/csv/1.1.5/csv/tutorial/index.html
