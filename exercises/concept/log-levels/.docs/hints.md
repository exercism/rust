# Hints

## General

- The [rust by example][rust-by-example-string] has a nice introduction to Rust `string`s.
- There are many useful [built-in methods][docs-string-methods].

## 1. Get message from a log line

- Use `String::new()` to create a new empy string.
- Use `mut` keyword to mark a variable mutable.
  ```rust
  let mut my_string = String::new();
  ```
- Use `.chars()` to iterate over the characters of a string. 
- Use `.trim()` on a string to remove whitespace.
- A `str` can be converted to `String` by using `.to_string()` method on it.


## 2. Get log level from a log line

- Use `.to_lowercase()` to change a string's case.


## 3. Reformat a log line

- Use `.push()` to push a charaacter in to a `String`.
- Use `.push_str()` to push a string in to a `String`.

[docs-string-methods]:
    https://doc.rust-lang.org/std/string/struct.String.html#implementations
[rust-by-example-string]: https://doc.rust-lang.org/rust-by-example/std/str.html?highlight=String#strings
