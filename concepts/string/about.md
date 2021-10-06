# String

The `String` data type is a linear series of characters encoded as UTF-8. It is useful for storing textual data.

## Creating a String

A String can be created using the `String::new()` or `String::from()` associated functions. Many types have a `to_string` method to produce a string e.g. from a number.

```rust
let my_string = String::from("The answer is");
let answer = 42.to_string();
println!("{} {}", my_string, answer);
```

### The `format!` Macro

Because producing a large string from a selection of smaller data types is a common operation, Rust also has a shorthand which elides the details of calling `to_string`: the `format!` macro. For example, we could simplify the construction of the composite string above as follows:

```rust
let the_answer = format!("{} {}.", "The answer is", 42);
assert_eq!("The answer is 42.", the_answer);
```

## String Slices

Strings have a close relationship with string slices. String slices are immutable references to a region of memory of known size, containing a linear series of characters encoded as UTF-8. Any string can be coerced into a string slice by borrowing with the dereference (`&`) operator. To convert a string slice to a string, the `to_string()` method or `String::from` are both appropriate.

In Rust code, a literal string is enclosed in double-quotes. These string literals are always string slices, because they refer to an immutable region of memory within the program's executable.

```rust
let string = String::from("string");
let slice = "string";
assert_eq!(slice, &string);
```

## Mutating a String

To concatenate a single character to a string, use the `push` method. To concatenate a string slice to a string, use the `push_str` method or the `+` operator.

```rust
fn main() {
    // For demonstration purposes, we're not using `format!`
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;
    println!("{}", s);
}
```

## Accessing Characters in a String

Since Rust strings are UTF-8 encoded individual characters are not all of the same size. Thus the `String` data type does not provide an indexing operator (`[]`). To access all the characters or a specific character in a string, the `chars()` method can be used. This iterates over each character in the string, in sequence.

```rust
let alphabet: String = ('a'..='z').collect();
for c in alphabet.chars() {
    println!("{}", c);
}
let j = alphabet.chars().nth(9);
println!("{:?}", j); // prints Some('j')
```

## More

String is a rich datatype with several methods and macros built around it. Check out [the standard library documentation](https://doc.rust-lang.org/std/string/struct.String.html) for all of the member methods and functionality provided by String.
