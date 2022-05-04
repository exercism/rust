# Methods

In Rust, methods are functions defined for a struct (or an `enum` or `trait` object).
Methods are similar to functions: they are declared with the `fn` keyword,
they have a name, and they can have parameters and return values.

But unlike functions, methods's first parameter is always `self`,
which represents the instance of the struct the method is being called on (more details below).

Let's create a struct that describes a wizard with the fields `name` and `house`:

```rust
struct Wizard {
    name: String,
    house: String,
}
```

## Defining methods

To define methods for the struct `Wizard`, we start an `impl` (short for implementation) block for `Wizard`.
Everything within this `impl` block will be associated with the `Wizard` type.
Note that a struct is allowed to have multiple `impl` blocks.

### Method with `&self`

If we wish to implement an `info` method to display the basic information of the wizard,
we could define this method inside an `impl` block for `Wizard`:

```rust
impl Wizard {    
    fn info(&self) {
        println!("{} studies in House {} at Hogwarts", self.name, self.house);
    }
}
```

Note that in the signature for `info`, we use `&self`, which is short for `self: &Self`.
The type `Self` is an alias for the type that the `impl` block is for, in this case `Wizard`.

Recall that we mentioned that a method must have a parameter named `self` of type `Self` as its first parameter,
so Rust allows us to abbreviate this with only the name `self` in the first parameter spot for convenience.

Note that we still need to use the `&` in front of the `self` shorthand to indicate this method
borrows the `Self` instance so that we could read the `name` and `house` fields of a `Wizard` instance.

With the `info` method implemented, we could read a wizard's basic information:
```rust
let harry = Wizard {
  name: String::from("Harry Potter"),
  house: String::from("Gryffindor"),
};

harry.info();
// => Harry Potter studies in House Gryffindor at Hogwarts
```

### Method with `&mut self`

Methods can borrow `self` immutably, as in `&self` for the signature of the method `info`).
However, if we wish to implement a method that mutates the fields of a struct instance, we must borrow the instance mutably.

Let's say we want to set up a new rule at Hogwarts to allow students to switch houses.
We could implement a method `switch_house` as following:

```rust
impl Wizard {
  fn switch_house(&mut self, new_house: String) {
    self.house = new_house;
  }
}
```

To enable a method to borrow `self` mutably, we simply use `&mut self` in the signature for the method.
Now we can switch houses for a wizard who studies at Hogwarts:

```rust
// Note that the variable `harry` must be mutable
let mut harry = Wizard {
  name: String::from("Harry Potter"),
  house: String::from("Gryffindor"),
};

harry.info();
// => Harry Potter studies in House Gryffindor at Hogwarts
harry.switch_house(String::from("Slytherin"));
harry.info()
// => Harry Potter studies in House Slytherin at Hogwarts
```
