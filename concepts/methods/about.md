# About

In Rust, methods are functions defined for a struct (or an `enum` or `trait` object).
This changes the calling pattern: instead of `frobnicate(foo)`, a method is called as `foo.frobnicate()`.

Methods are similar to functions: they are declared with the `fn` keyword,
they have a name, and they can have parameters and return values.

Unlike functions, a methods's first parameter is always `self` (or a variant),
which represents the instance of the struct the method is being called on (more details below).

Let's create a `enum` for Hogwarts's houses and a `struct` that describes a wizard's `age` and `house`:

```rust
#[derive(Debug)]
enum House {
    Gryffindor,
    Hufflepuff,
    Ravenclaw,
    Slytherin,
}

struct Wizard {
    age: u32,
    house: House,
}
```

## Defining methods

To define methods for the struct `Wizard`, we start an `impl` (short for implementation) block for `Wizard`.

```rust
impl Wizard { }
```

Everything within this `impl` block will be associated with the `Wizard` type.
Note that a struct is allowed to have multiple `impl` blocks.

## Using `self`

A method must have a parameter named `self` of type `Self` as its first parameter.
The type `Self` is an alias for the type that the `impl` block is for.
In our example, `Self` represents `Wizard`.

Rust allows us to abbreviate this with only the name `self` in the first parameter spot for convenience.
Common `self` types for methods include:

- `self`: Abbreviates `self: Self`. The bare `self` signature consumes the instance
- `&self`: Abbreviates `self: &Self`. This borrows the instance immutably.
- `&mut self`: Abbreviates `self: &mut Self`. This borrows the instance mutably.

In more advanced code you might also see signatures such as `self: Pin<&self>`, `self: Rc<Self>`, etc.
This is, however, beyond the scope of the current tutorial.

### Defining method with `&self`

If we wish to implement an `info` method to display the basic information of the wizard,
we could define this method inside an `impl` block for `Wizard`:

```rust
impl Wizard {    
    fn info(&self) {
        println!(
            "A wizard of age {} who studies in House {:?} at Hogwarts",
            self.age,
            self.house,
        );
    }
}
```

Note that in the signature for `info`, we borrow the instance immutably with `&self`.
This way, we could read the `age` and `house` fields of a `Wizard` instance without changing them.

With the `info` method implemented, we could read a wizard's basic information:
```rust
let harry = Wizard { age: 11, house: House::Gryffindor };
harry.info();
// => A wizard of age 11 who studies in House Gryffindor at Hogwarts
```

### Defining method with `&mut self`

If we wish to implement a method that mutates the fields of a struct instance,
we borrow the instance mutably with `&mut self` in the signature for the method.

Let's say we want to set up a new rule at Hogwarts to allow students to switch houses.
We could implement a method `switch_house` as following:

```rust
impl Wizard {
    fn switch_house(&mut self, new_house: House) {
        self.house = new_house;
    }
}
```

Now we can switch houses for a wizard who studies at Hogwarts:

```rust
// Note that the variable must be mutable
let mut cedric = Wizard::new(14, House::Hufflepuff);
cedric.info();
// => A wizard of age 14 who studies in House Hufflepuff at Hogwarts
cedric.switch_house(House::Slytherin);
cedric.info()
// => A wizard of age 14 who studies in House Slytherin at Hogwarts
```

### Multiple 'impl' blocks

In Rust, each struct is allowed to have multiple `impl` blocks.
So we can group all methods in the same `impl` block or have multiple `impl` blocks for different groups of methods.
Both approaches are valid syntax, but typical project organization uses a single `impl` block per unique set of `impl`-level `where` bounds.

## Associated Functions

It is possible to define functions within an `impl` block which do not contain a `self` parameter.
Because they lack the `self` parameter, they are not methods.
However, they are still associated with the implemented type, and called by naming that type.
For that reason, they are called associated functions (or type-associated functions).

You have already seen some associated functions, such as `String::from()`:
```rust
let some_string = String::from("This is an associated function but not a method");
```

Constructors are often implemented as associated functions.
Let's create a `new` function associated with the `Wizard` type:

```rust
impl Wizard {
    fn new(age: u32, house: House) -> Self {
        Self {
            age,
            house,
        }
    }
}
```

Note that we can use `Self` instead of `Wizard` to indicate the type that will be returned from the function.
It is not mandatory to use the `Self` type alias in this way, but it can simplify the implementation for types with complicated or verbose names.
Now we can run the following:

```rust
let myrtle = Wizard::new(14, House::Ravenclaw);

myrtle.info();
// => A wizard of age 14 who studies in House Ravenclaw at Hogwarts
```
