# Mutability

Mutability is a property of a particular binding in Rust. A binding is either immutable or mutable. An immutable binding's value may not be changed; a mutable binding's value can be changed. All bindings are immutable by default.

## Bindings

A binding is the association between a particular variable name as it appears in the source code and a particular region of memory as it appears during execution. Strictly speaking, bindings only exist during compilation: before compilation, there is no memory actually allocated, and after compilation, the machine code deals exclusively with memory addresses instead of named variables.

Bindings are typically created with the `let` keyword. Additionally, bindings are created from function parameters and in certain other circumstances.

Bindings can have various properties, the most important of which is mutability. A mutable binding is one for which it is legal to change the value of the bound memory. Rust strictly tracks and enforces the mutability of each binding to help ensure that programs are correct, by rejecting programs which incorrectly attempt to modify a binding which should be immutable.

## Mutability

In practice, mutability is often simple. By using the `mut` keyword in the binding's definition, you get the ability to change the value.

```rust,ignore
let x = 1;
// this does not compile because `x` is immutable
x = 2;
```

```rust
let mut x = 1;
x = 2;
// 🎉🎆🎉🎆🎉
```

### Demoting from Mutable to Immutable

A binding can always be demoted from mutable to immutable. This operation is transparent.

```rust
fn debug(n: i32) {
    dbg!(n);
}

fn main() {
    let mut x = 1;
    x += 2;
    debug(x);                // emits "n = 3"
}
```

### Promoting from Immutable to Mutable

In general, it is possible to promote a value from immutable to mutable any time that ownership is transferred.

```rust
let x = "hello".to_string();
let mut y = x;               // (1)
y += ", world";
dbg!(y);
```

At `(1)`, ownership of the string is transferred from `x` to `y`, which allows us to promote the value to mutable.

### Mutability in Function Parameters

There are two different ways for function parameters to be mutable. The first and more common method resembles passing by reference in C:

```rust
fn increment(n: &mut i32) {  // (1)
    *n += 1;
}

fn main() {
    let mut x = 1;           // (2)
    increment(&mut x);       // (3)
    dbg!(x);                 // emits "x = 2"
}
```

In this example, the mutability was explicitly annotated in three places. All three of the mutability declarations are required in order for mutable references to work.
In `(2)`, the binding `x` is declared to be mutable. This binding is the top-level owner of the value.
In `(3)`, that value is mutably borrowed, to pass into the `increment` function.
In `(1)`, a new, mutable binding `n` is created to the appropriate reference.

There is another style of parameter mutability, which is less common but still sometimes useful. In this style, the value is owned, but the binding is mutable.

```rust
fn increment_internally(mut n: i32) {   // (1)
    n += 1;
    dbg!(n);                            // emits "n = 2"
}

fn main() {
    let x = 1;                          // (2)
    increment_internally(x);            // (3)
    dbg!(x);                            // emits "x = 1"
}
```

This example requires fewer explicit annotations because it's actually a transfer of ownership with built-in mutability promotion. It works because `i32` is a `Copy` type which copies data transparently.
In `(1)` we declare that `increment_internally` accepts a mutable owned `i32` named `n` as its parameter.
In `(2)` we declare `x` to be an immutable binding to the value `1`.
In `(3)` `x` is copied into a new location in memory. `x` remains immutable. `n` is mutable.

It is of course possible to combine the forms, giving a mutable binding to a mutable reference (`mut n: &mut i32` etc.), but this is rarely what's actually desired.

### Mutability Without `let`

Several constructions in Rust (`if let`, `match`, etc.) can also create bindings. These bindings can also be mutable.

```rust
fn maybe_increment(n: &mut Option<i32>) {
    if let Some(ref mut n) = n {
        *n += 1;
    }
}

fn main() {
    for mut v in [Some(1), None] {
        maybe_increment(&mut v);
        dbg!(v);
    }
}
```

This example produces first `v = Some(2)`, then `v = None`.

## Shadowing

It's possible, and often idiomatic, to reuse a variable name as a particular value is transformed, by re-declaring it with multiple `let` invocations. This is known as shadowing.

```rust
let s = "my string";
let s = s.to_uppercase();
dbg!(s);                               // emits "s = MY STRING"
let s = s.split_whitespace().count();
dbg!(s);                               // emits "s = 2"
```

However, this is not a case of mutability; the original values are in principle still present. Given non-`Drop` types, the compiler may choose to mutate a memory location instead of allocating a new one. However, it will never do this given a `Drop` type or if the old binding is used again.

```rust
let x = 1;

// creating a scope like this forces bindings created internally to drop
// when the scope is exited
{
    let x = 10;
    dbg!(x);                   // emits "x = 10"
}

dbg!(x);                       // emits "x = 1"
```
