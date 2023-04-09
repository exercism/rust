# Introduction

## Strings

There are two types of strings in Rust: `String` and `&str`. A &str is a slice,
which can be usd to view in to a `String. More on this in later concepts.

Creating a string literal produces a `&str`.
```rust
let my_string = "Hello!"; // &str
```

You can create a `String` from a literal string with `String::from`:
```rust
let hello = String::from("Hello, world!");
```

Or you can also use `.to_string()`:
```rust
let hello = "My awesome string".to_string(); // String
```

For converting a `String` to a `&str` prefix the variable with `&`. This is know
as borrow operator. More on that in later concepts.
```rust
let awesome_string = String::from("Awesome str"); // String
let my_str = &awesome_string; // &str
```


You can append a `char` to a String with the `push` method, and append a `&str` with
the `push_str` method:
```rust
let mut hello = String::from("Hello, ");

hello.push('w');
hello.push_str("orld!");
```

Notice in the above example, in order to make a change to a string, you have to
mark the variable as mutable using `mut` keyword.

### Built-in methods
Various utility methods are available on a `String`.

1. `.trim()` & `.to_lowercase()`
    ```rust
    let hello = "My spacy string   ".trim() // => "My spacy string"
    let hello = "HELLO".to_lowercase() // => "hello"
    ```

1. Characters in a string can be iterated over using for loops and the `.chars()`
method:
    ```rust
    let my_str = "strings are fun";

    for character in my_str.chars() {
        // do something
    }
    ```

1. `.contains()` can be used to check if given input is part of a string
    ```rust
    let bananas = "bananas";

    bananas.contains("nana"); // => true
    ```
    Similarly, `.start_with()` and `.ends_with()` can be used to check if the
    input is at the beginning or end of a string.
