# Introduction

In Rust, there are are many specific types of numbers but they generally fall under one of two categories: integers or floating-point numbers.

Initializing an integer:
```rust
let x = 5; // rust defaults to an i32
```

Initializing a large positive integer:
```rust
let y: u64 = 7_000_000_000; // unsigned numbers can't be negative
```

Initializing floating-point numbers:
```rust
let x = 1.2; // defaults to f64

let y: f32 = 5.0; // initializing an f32
```
