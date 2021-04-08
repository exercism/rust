# About

There are two ways that Rust implements an array of characters: [`str`][str] and [`String`][String].

`str`, also known as a "string slice", is usually used with a reference as `&str`.

To know why, it may help to understand why the following line of code will not compile

```rust
const MY_CONST: str = *"Hello, World!";

// the size for values of type `str` cannot be known at compilation time
// the trait `Sized` is not implemented for `str`
```

Even though the literal "Hello, World!" length can be known at compilation time, as a `str` it is a sequence of unknown size.
But as a _reference_ to a literal (`&str`), it has a "fat pointer" which holds the address and length of the literal in memory.
Fat pointers are of a known size, and so this line will compile

```rust
const MY_CONST: &str = "Hello, World!";
```

A mutable binding to a string slice can have its valued replaced, but not modified in place.

Example

```rust
let mut my_str = "Hello, World!";
my_str = "Hi!";
let mut my_string = String::from("Hello");
my_string.push_str(", World!"); // there is no equivalent function to modify &str
```

It is less constraining to use `&str` instead of `&String` when defining a parameter for a function, as this will not compile

```rust
pub fn main() {
    let my_str: &str = "Hello, World!";
    say_hi(my_str);
}

fn say_hi(phrase: &String) {
    println!("{:?}", phrase);
}

// mismatched types
// expected reference `&String`
// found reference `&str`
```

A `&str` will not be [coerced][coercion] into a `&String`. But this will compile

```rust
pub fn main() {
    let my_string = String::from("Hello, World!");
    say_hi(&my_string)
}

fn say_hi(phrase: &str) {
    println!("{:?}", phrase);
}
```

A `&String` will be coerced into a `&str`. So to use a parameter type of `&str` will often be more useful.

[str]: https://doc.rust-lang.org/std/primitive.str.html
[String]: https://doc.rust-lang.org/std/string/struct.String.html
[coercion]: https://doc.rust-lang.org/reference/type-coercions.html
