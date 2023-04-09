# Introduction

Rust doesn’t have the null feature that many other languages have. Null is a
value that means there is no value there. In languages with null, variables can
always be in one of two states: null or not-null.

The problem with null values is that if you try to use a null value as a not-null value, you’ll get an error of some kind. Because this null or not-null property is pervasive, it’s extremely easy to make this kind of error.

Sometimes it's desirable to represent an _absence of a value_ or _invalid
value_. In Rust, this is accomplished using `Option<T>` enum.

## Option

To encode the concept of value being present or absent the standard library
defines `Option<T>` enum as follows.

```rust
enum Option<T> {
    None,
    Some(T),
}
```

Its variants are included in the prelude: you can use `Some` and `None`
directly without the `Option::` prefix.

The `<T>` syntax is a feature of Rust we haven’t talked about yet. It’s a generic
type parameter, and we’ll cover generics in more detail in later concepts.  

For now, all you need to know is that 
- `<T>` means that the `Some` variant of the `Option` enum can hold one piece of
  data of any type,
- Each type that gets used in place of `T` makes the overall `Option<T>` a
  different type.

Here are some examples of using `Option` values to hold number types and string types:

```rust
    let some_number = Some(5); // type Option<i32>
    let some_char = Some('e'); // type Option<char>
    let some_str = Some("Hello"); // type Option<&str>

    // None does not have the <T> syntax, which means 
    // we have to explicitly write the type for this variable
    let absent_number: Option<i32> = None; 
```

There are multiple ways to use the value held by the `Some` variant. For now we
will use the `.unwrap()` method.

```rust
let maybe_number = Some(10);
let my_number = 5;

my_number + maybe_number.unwrap() // => 15
```

In the above example, `10` cannot be directly added to `5`, it must first be
unwrapped. Doing so otherwise will throw a panic.  
As mentioned previously, the
`T` syntax makes the overall `Option<T>` a different type. Without the
`.unwrap()` we would be trying to add `i32` with `Option<i32>` which is not supported.

One can also use `if` statements to run different code based on whether a value
is present or not.

```rust
let my_name: Option<&str> // Assume there is a variable, we don't know it's value yet.

let mut greeting = "Hello, World!";

if my_name != None {
    greeting = &format!("Hello, {}!", my_name.unwrap());
}

// If my_name was None
// greeting => "Hello, World!"
// If my_name was Some("John")
// greeting => "Hello, John!"

```

If we write below code without the if statement guard, and `my_name` was `None`.
The program would panic.
```rust
greeting = &format!("Hello, {}!", my_name.unwrap());

// => 'called `Option::unwrap()` on a `None` value'
```
