# Hints

## 1. Print a badge for an employee

- [String formatting][string-formatting] can be used to concisely format the badge.
- There is a [built-in method to convert a string to uppercase][to_uppercase()].

## 2. Print a badge for a new employee

- You should check if the ID is `None` before using it.
- Use `.unwrap()` to get the value of ID.

## 3. Print a badge for the owner

- You should check if the department is `None` before using it.

[string-formatting]: https://doc.rust-lang.org/rust-by-example/hello/print/fmt.html
[to_uppercase()]: https://doc.rust-lang.org/std/string/struct.String.html#method.to_uppercase
