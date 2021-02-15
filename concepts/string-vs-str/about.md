# About

`String` and `&str` are two fundamental ways of working with textual data in Rust.
Knowing their characteristics and how they work together comes in very handy.
This document discusses how they overlap and offers some basic advice.
`String` is an owned, vector of `u8` bytes.
`String` is mutable.
`&str`, is a borrowed reference to a String slice.
`&str` is pronounced "and stir"
`&str` is immutable.

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
`&str` borrows textual data so will perform better at the tradeoff of your struct needing to define lifetime parameters.
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
