# Introduction

Rust's vector implementation, [`std::vec::Vec`](https://doc.rust-lang.org/std/vec/struct.Vec.html), can be used as a stack by using its `push()` and `pop()` methods.
`push()` adds to the stack and `pop()` removes from it.
Stacks are a useful idiom to store and access arbitrary numbers of variables one at a time.
One situation where a stack is required is evaluating Reverse Polish notation expressions.
