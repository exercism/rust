# String

String datatype represents a sequence of characters commonly used to store text or textual data. A String in rust is UTF-8 encoded and thus has the ability to store text in all Unicode-support languages and character-sets. This also means that each character of a string needs multiple bytes to store (typically four).

## Creating a string

A String is created using the `String::new()` or `String::from()` method. Several types support the `to_string` methods to convert into a string e.g., from a number.

```rust
pub fn main() {
    let my_string = String::from("The answer is");
    let answer = 42.to_string();
    println!("{} {}", my_string, answer);
}
```

## String literals aka String slices

Strings have a close relationship with string-slices or `str`. In rust code, a literal string is enclosed in double-quotes. These are known as string-slices, `str` for short, and are immutable (non-modifiable). They represent a reference to a string stored somewhere else in the memory.

```rust
pub fn main() {
    let my_str = "A sequence of characters";
    println!("{}", my_str);
}
```

Strings on the other hand are mutable and own the characters stored inside. Strings can be converted to string-slices using the dereference (`&`) operator. To make a string from a string-slice use the `String::from()` function.

## Updating a string

To add characters to the end of a string use `push` or `push_str` method. The addition or concatenation operator (`+`) is used to add a string slice to a string.

```rust
fn main() {
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;
    println!("{}", s);
}
```

## Accessing characters in a string

Since rust strings are UTF-8 encoded individual characters are not all of the same-size. Thus the String datatype in rust does not provide an indexing operator (`[]`). To access all the characters or a specific character in a string, the `chars()` member function can be used.

```rust
fn main() {
    let alphabet = String::from("abcdefghijklmnopqrstuvwxyz");
    for c in alphabet.chars() {
        println!("{}", c);
    }
    let j = alphabet.chars().nth(9);
    println!("{:?}", j); // prints Some('j')
}
```

String is a rich datatype with several methods and macros built around it. Check out [the standard-library documentation](https://doc.rust-lang.org/std/string/struct.String.html) for all of member methods and functionality provided by String.
