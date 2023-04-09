# Introduction

A `struct`, or structure, is a custom data type that lets you package together and name multiple related values that make up a meaningful group. If you’re familiar with an object-oriented language, a struct is like an object’s data attributes.

## Struct

Structs let you create custom types that are meaningful for your domain. By
using structs, you can keep associated pieces of data connected to each other
and name each piece to make your code clear. In `impl` blocks, you can define
functions that are associated with your type, and methods are a kind of
associated function that let you specify the behavior that instances of your
structs have.


There are three types of structures ("structs") that can be created using the
`struct` keyword:

- Tuple structs, which are, basically, named tuples.
- The classic [C structs][c-structs]
- Unit structs, which are field-less, are useful for generics.

```rust
// A unit struct
struct Unit;

// A tuple struct
struct Pair(i32, f32);

// A struct with two fields
struct Point {
    x: f32,
    y: f32,
}
```

For the purposes of this exercise, we will focus on C structs.

### Instantiating a struct
To use a struct after we’ve defined it, we create an instance of that struct by
specifying concrete values for each of the fields

We create an instance by stating the name of the struct and then add curly
brackets containing key: value pairs, where the keys are the names of the fields
and the values are the data we want to store in those fields. We don’t have to
specify the fields in the same order in which we declared them in the struct.

```rust
let my_point = Point { x: 1, y: 1 };
```

### Access struct fields
To get a specific value from a struct, we use dot notation. For example, to
access this point's x value we use `my_point.x`. If the instance is mutable, we
can change a value by using the dot notation and assigning into a particular
field.

```rust
let mut my_point = Point { x: 1, y: 1 };
my_point.x += 1; // => x is now 2
```

### Use as type

Structs can be used as variable types, function parameter types as well as
return types of a function.

```rust
let my_var: Point;

fn my_function(input: Point) -> Point {}
```

### Defining Methods
Methods are similar to functions: we declare them with the fn keyword and a
name, they can have parameters and a return value, and they contain some code
that’s run when the method is called from somewhere else.

Their first parameter is always `self`, which represents the instance of the
struct the method is being called on.

Lets start with an example:

```rust
struct Rectangle {
    width: u32,
    height: u32,
}
```

To define the function within the context of `Rectangle`, we start an `impl`
(implementation) block for `Rectangle`. Everything within this `impl` block will be
associated with the `Rectangle` type. Then we move the area function within the
`impl` curly brackets and change the first (and in this case, only) parameter to
be `self` in the signature and everywhere within the body. 

```rust
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}
```

### Calling methods

To call `area` we use the method syntax which goes after an instance: we add a dot followed by the method name, parentheses, and any arguments.

```rust
let rect1 = Rectangle {
    width: 30,
    height: 50,
};

rect1.area() // => 1500
```

### Method parameters

In the signature for `area`, we use `&self`. The `&self` is actually short for `self:
&Self`. Within an `impl` block, the type `Self` is an alias for the type that the
`impl` block is for. Methods must have a parameter named `self` of type `Self` for
their first parameter, so Rust lets you abbreviate this with only the name `self`
in the first parameter spot.

If we wanted to change the instance that we’ve called the method on as part of
what the method does, we’d use `&mut self` as the first parameter.

```rust
impl Rectangle {
    fn update_width(&mut self, new_width: u32) {
        self.width = new_width;
    }
}
```


### Associated Functions

All functions defined within an `impl` block are called associated functions
because they’re associated with the type named after the `impl`. We can define
associated functions that don’t have `self` as their first parameter (and thus are
not methods) because they don’t need an instance of the type to work with.

Associated functions that aren’t methods are often used for constructors that
will return a new instance of the struct. These are often called `new`, but `new`
isn’t a special name and isn’t built into the language. For example, we could
choose to provide an associated function named `square` that would have one
dimension parameter and use that as both `width` and `height`, thus making it easier
to create a square `Rectangle` rather than having to specify the same value twice:

```rust
impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}
```

The `Self` keywords in the return type and in the body of the function are aliases for the type that appears after the `impl` keyword, which in this case is Rectangle.

To call this associated function, we use the :: syntax with the struct name:
```rust
let my_square = Rectangle::square(10);
```

[c-structs]: https://en.wikipedia.org/wiki/Struct_(C_programming_language)
