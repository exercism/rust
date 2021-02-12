# Introduction

It is often useful to group a collection of items together, and handle those groups as units. In Rust, we call such a group a struct, and each item one of the struct's fields. A struct defines the general set of fields available, but a particular example of a struct is called an instance.

Furthermore, structs can have methods defined on them, which have access to the fields. The struct itself in that case is referred to as `self`. When a method uses `&mut self`, the fields can be changed, or mutated. When a method uses `&self`, the fields cannot be changed: they are immutable. Controlling mutability helps the borrow-checker ensure that entire classes of concurrency bug just don't happen in Rust.

In this exercise, you'll be implementing two kinds of methods on a struct. The first are generally known as getters: they expose the struct's fields to the world, without letting anyone else mutate that value. In Rust, these methods idiomatically share the name of the field they expose, i.e., if we have a getter method that fetches a struct field called `name`, the method would simply be called `name()`.

You'll also be implementing methods of another type, generally known as setters. These change the value of the field. Setters aren't very common in Rust--if a field can be freely modified, it is more common to just make it public--but they're useful if updating the field should have side effects, or for access control: a setter marked as `pub(crate)` allows other modules within the same crate to update a private field, which can't be affected by the outside world.

Structs come in three flavors: structs with named fields, tuple structs, and unit structs. For this concept exercise, we'll be exploring the first variant: structs with named fields.

Structs are defined using the `struct` keyword, followed by the capitalized name of the type the struct is describing:

```rust
struct Item {}
```

Additional types are then brought into the struct body as _fields_ of the struct, each with their own type:

```rust
struct Item {
    name: String,
    weight: f32,
    worth: u32,
}
```

Lastly, methods can be defined on structs inside of an `impl` block:

```rust
impl Item {
    // initializes and returns a new instance of our Item struct
    fn new() -> Self {
        unimplemented!()
    }
}
```

With that brief introduction to the syntax of structs out of the way, go ahead and take a look at the [instructions](instructions.md) for this exercise!
