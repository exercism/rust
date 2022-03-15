# Introduction

## Null-References

If you have ever used another programming language (C/C++, Python, Java, Ruby, Lisp, etc.), it is likely that you have encountered `null` or `nil` before. 
The use of `null` or `nil` is the way that these languages indicate that a particular variable has no value. 
However, this makes accidentally using a variable that points to `null` an easy (and frequent) mistake to make. 
As you might imagine, trying to call a function that isn't there, or access a value that doesn't exist can lead to all sorts of bugs and crashes. 
The creator of `null` went so far as to call it his ['billion-dollar mistake'.](https://www.infoq.com/presentations/Null-References-The-Billion-Dollar-Mistake-Tony-Hoare/)

## The `Option` Type

To avoid these problems, Rust does not use null-references.
However, it still needs a safe way to indicate that a particular variable has no value.
This is where `Option` comes in.
Instead of having a variable which lacks a value, Rust variables can use the `Option` enum.
This enum has two variants: `None`, Rust's null-equivalent; and `Some(T)`, where T is a value of any type.

It looks like this:

```rust
enum Option<T> {
    None,
    Some(T),
}
```

You can think of `Option` as a layer of safety between you and the problems that null-references can cause, while still retaining their conceptual usefulness.

## Using `Option`

Setting a variable to `None` is fairly straightforward:

```rust
let nothing: Option<u32> = None;  // Variable nothing is set to None
```

However, if you wish for the `Option` type to carry a value, you cannot assign this value directly.
An `Option` type variable and, say, an `i32` type variable are not equivalent.
You will need to use `Some`:

```rust
let wrong_way: Option<i32> = -4; // This will not work

let right_way: Option<i32> = Some(-4); // This will work
let another_right_way = Some(-4); // Compiler infers that this is Option<i32>
```

It's also for this reason that the following will not work:

```rust
let number = 47;
let option_number = Some(15);

let compile_error = number + option_number; // Cannot add an i32 and an Option<i32> - they are of different types
```

If you wish to get the value that is contained by Some, you will first need to check that it exists:

```rust
let mut some_words = Some("choose something to say"); // some_words set to something

match some_words {
    Some(str) => println!("Here, we will {}", str),
    None => println!("I've got nothing to say"),
} // Prints "Here, we will choose something to say"

some_words = None; // some_words now set to None

// exactly the same match block as above
match some_words {
    Some(str) => println!("Here, we will {}", str),
    None => println!("I've got nothing to say"),
} // Prints "I've got nothing to say"
```

Besides `match`, Rust has other tools available for checking and accessing values contained within `Option`, but `match` should be familiar to you by now.

Additionally, consider this a demonstration of why Rust uses `Option` instead of a null-reference.
The point is that **you _must_ check** whether or not the `Option` variable is `Some` (in which case you can go ahead and extract and use the value contained within), or `None`.
Anything else, and your program will not compile; the compiler is keeping you safe from `null`.
