## Implementing Recursive Data Structures in Rust

Unlike other languages, implementing a seemingly simple data structures like linked list or binary search tree
is not trivial in Rust and more so in the `safe` subset of Rust that does not use operations on the raw pointers
and you quickly may end up with structs that contain fields with beautiful types like `Option<Rc<RefCell<Box<Node>>>>`.

In this exercise we ask you to implement a simple Binary Search Tree using raw pointers from the [`unsafe` subset of Rust](https://doc.rust-lang.org/book/second-edition/ch19-01-unsafe-rust.html#dereferencing-a-raw-pointer). The main thing to remember about `unsafe` Rust is that it is basically a normal mode
in other languages that deal with manual memory management like C, where the compiler does not make any predictions whether your code can cause a memory leak or
a segfault and the safety of your program is entirely up to you. While the usage of `unsafe` Rust is sometimes inevitable, it is preferred to keep this usage to a minimum
(there is even a project to measure the amount of your unsafe code - [cargo-geiger](https://github.com/anderejd/cargo-geiger)).

To lear more about implementing recursive data structures in Rust refer to the [following tutorial](http://cglab.ca/~abeinges/blah/too-many-lists/book/).
