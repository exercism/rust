This concept exercise gave you some practice writing structs and methods in Rust.

One of the important takeaways from this exercise is being able to tell when we need an _immutable_ reference to our struct instance via `&self` vs. when we need a _mutable_ reference to our struct instance via `&mut self`.

Notice that the getter methods made use of `&self` since they are only concerned with reading data from our struct instance. The setter methods, in contrast, made use of `&mut self` since they are concerned with updating our struct instance.

It should also be noted that the usage of getter and setter methods in this concept exercise was a bit artificial in order to expose a use-case for `&self` vs. `&mut self` methods. In real projects, the fields would simply be made public, such that we'd have a struct like:

```rust
pub struct User {
    pub name: String,
    pub age: u32,
    pub weight: f32,
}
```

This way, we can fetch and/or update fields on structs without having to go through a function call.

There are also other struct variants that this exercise didn't touch on, namely _tuple structs_ and _unit structs_.

- [https://learning-rust.github.io/docs/b2.structs.html#Tuple-structs](https://learning-rust.github.io/docs/b2.structs.html#Tuple-structs) highlights the use-cases of tuple and unit structs.

- [https://doc.rust-lang.org/stable/rust-by-example/custom_types/structs.html](https://doc.rust-lang.org/stable/rust-by-example/custom_types/structs.html) in _Rust By Example_ showcases some examples of tuple and unit structs.
