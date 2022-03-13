# Introduction

In Rust, assigning a value to a name is referred to as a _binding_. Bindings are immutable unless declared with the `mut` keyword. As Rust is a statically-typed language, each binding has a type known at compile-time.

Bindings are most commonly defined using the `let` keyword. Specifying a binding's type is optional for most bindings, as Rust's _type inference_ can usually infer the type based on their value. A binding looks like this:

```rust
// Automatically inferred type
let fingers = 10;
```

Functions are _items_. Where bindings typically refer to a particular value, items refer to a unit of code organization, typically a function or a module, which is available throughout the lifetime of the program. A function automatically returns the result of its last expression. A function may have 0 or more parameters, which are bindings with a lifetime of the function call.

Type inference is theoretically possible for functions, but is disabled as an intentional language design choice. While this means that you need to spend a little more time when writing code to specify precisely what a function's input and output types are, you save the time when you're reading the code, because all the input and output types are explicitly defined.

```rust
fn add(x: i32, y: i32) -> i32 {
  x + y
}
```

Invoking a function is done by specifying its name followed by parentheses. If the function requires parameters, an argument must be specified for each within the parentheses.

```rust
let five = add(2, 3);
```

If a binding's type cannot be inferred, the compiler will report an error. To fix this, add an explicit type annotation to the binding.

```rust
// Explicit type annotation
let fingers: i32 = 10;
```

Items in Rust can be used before or after they are defined, because they have a static lifetime. Bindings, on the other hand, can only be used _after_ they have been defined. Using a binding before it has been defined results in a compile error.

```rust
fn main() {
    // `fn add` hasn't yet been defined, but that's perfectly ok
    dbg!(add(3, 4));
}

fn add(x: i32, y: i32) -> i32 {
  x + y
}
```

```rust
// this won't compile; `a` is used before its binding is defined
let b = a;
let a = x + y;
```

Rust uses curly braces (`{}`) to define a scope. A binding defined within a scope can't escape from it.

```rust
let a = 1;
dbg!(a); // 1
{
    // Here, we re-bind `a` to a new value, which is still immutable.
    // This technique is called _shadowing_. The new binding is constrained to
    // this anonymous scope. Outside this scope, the previous binding still
    // applies.
    let a = 2;
    let b = 3;
    dbg!(a, b); // 2, 3
}
// can't use `b` anymore because it is out of scope
// dbg!(b);

// The shadowed `a` in the inner scope above has fallen out of scope,
// leaving us with our original binding.
dbg!(a); // 1
```

Rust items are often organized in modules. Each crate is implicitly a module, but it can define inner sub-modules of arbitrary depth. A module groups related functionality and is defined using the `mod` keyword.

```rust
mod calc_i32 {
    fn add(a: i32, b: i32) -> i32 { a + b }
    fn sub(a: i32, b: i32) -> i32 { a - b }
    fn mul(a: i32, b: i32) -> i32 { a * b }
    fn div(a: i32, b: i32) -> i32 { a / b }
}
```

Rust supports two types of comments. The keyword `//` indicates a single-line comment; everything following the keyword until the end of the line is ignored. The keywords `/*` and `*/` indicate a multi-line comment; everything within those two keywords is ignored. It is idiomatic and good practice to prefer single-line comments.

Rust also supports doc-comments, which show up in the generated documentation produced by `cargo doc`. Outer doc comments are formed with the keyword `///`, which acts identically to the `//` keyword. They apply to the item which follows them, such as a function:

```rust
/// The `add` function produces the sum of its arguments.
fn add(x: i32, y: i32) -> i32 { x + y }
```

Inner doc comments are formed with the keyword `//!`, which acts identically to the `//` keyword. They apply to the item enclosing them, such as a module:

```rust
mod my_cool_module {
    //! This module is the bee's knees.
}
```

Doc comments can be of arbitrary length and contain markdown, which is rendered into the generated documentation.
