# Introduction

## Mutability

Variable bindings in Rust are immutable by default. This just means that you can't change the associated value. Both direct assignment (`x = 1`) and changes (`x += 1`) are prohibited.

To opt in to mutability, just use the `mut` keyword.

```rust
let mut x = 1;
x = 2;
// 🎉🎆🎉🎆🎉
```

## Loops

A loop is the general term for when a program executes the same section of code repeatedly. The one simple type of loop in Rust is the `while` loop. This loop checks a condition before executing each iteration. If the condition is true, an iteration is executed, then the condition is checked again. The first time the condition evaluates to false, the loop is exited and control resumes after the loop body.

```rust
let mut i = 0;
while i < 5 {
    dbg!(i);
    i += 1;
}
```

In the example above, the loop body, contained within the curly brackets, is executed five times. The first time, `i` has the value `0`; the second, it has the value `1`, and so forth. Note that this means that in the final iteration `i` has the value `4`.

There are other forms of loops; see concept documentation for details.
