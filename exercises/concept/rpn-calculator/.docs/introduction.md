# Introduction

[Stacks](https://en.wikipedia.org/wiki/Stack_%28abstract_data_type%29) are a type of collection commonly used in computer science.
They are defined by their two key operations: **push** and **pop**.
**Push** adds an element to the top of the stack.
**Pop** removes and returns the topmost element.

Think of a stack like a stack of plates.
You can either add a plate to the top of the stack or take the topmost plate.
To access something further down, you have to remove all of the plates above it.

Rust's vector implementation, [`std::vec::Vec`](https://doc.rust-lang.org/std/vec/struct.Vec.html), can be used as a stack by using its `push()` and `pop()` methods.
Naturally, `push()` adds an element to the end of the `Vec` and `pop()` removes and returns the last element.
These operation can be very fast (O(1) in [Big O Notation](https://en.wikipedia.org/wiki/Big_O_notation)),
so they are one of the most idiomatic ways to use a `Vec`.

Stacks are useful to hold arbitrary numbers of elements in a specific order.
Because the last element inserted is the first element returned,
stacks are commonly refered to as **LIFO** (Last-In, First-Out).
This inherent ordering can be used for many things,
including tracking state when evaulating **Reverse Polish notation**.
