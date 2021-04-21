# Introduction

Stacks are a useful idiom to store and access arbitrary numbers of variables one at a time.
Think of a stack of plates.
You can only access the very top of the stack,
either to add to it or remove from it.
However, you can carry around a large number of plates when they are all stacked up.

Rust's vector implementation, [`std::vec::Vec`](https://doc.rust-lang.org/std/vec/struct.Vec.html), can be used as a stack by using its `push()` and `pop()` methods.
`push()` adds to the stack and `pop()` removes from it.
These operation can be very fast (O(1) in [Big O Notation](https://en.wikipedia.org/wiki/Big_O_notation)),
so they are one of the most idiomatic ways to use a `Vec`.

One situation in which a stack is required is evaulating Reverse Polish notation.
