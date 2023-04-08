# Hints

## General

- [String formatting][string-formatting]: tutorial on how to use string formatting.
- [std::fmt]: Utilities for formatting and printing Strings.


## 1. Display the couple's name separated by a heart

- Take a look at the [width] & [fill/alignemnt] part of [std::fmt].
- An argument of `usize` must be passed to define the width. You can convert a
  `u8`, `i32` or any other integers to `usize` as follows
  ```rust
  // Using .into()
  let my_width: usize = 6.into();
  // Using casting
  let my_width2 = 6 as usize;
  ```

## 2. Display the couple's initials in an ascii art heart

- Format strings cannot be defined in variables or constants. They must be
  passed directly to the `format!()` macro.
- You can define multiline strings without any special syntax.
- You can remove extraneous whitespace using `.trim()` method on a string.


[string-formatting]: https://doc.rust-lang.org/rust-by-example/hello/print/fmt.html
[std::fmt]: https://doc.rust-lang.org/std/fmt/
[fill/alignemnt]: https://doc.rust-lang.org/std/fmt/#fillalignment
[width]: https://doc.rust-lang.org/std/fmt/#width
