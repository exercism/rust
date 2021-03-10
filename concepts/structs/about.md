# Structs

Structs collect a fixed, heterogeneous set of data into a single unit. Structs in Rust have these properties:

- Every struct type has a particular name.
- Every member of a struct is called a **field**.
- Every field of a struct has a particular name.
- Visibility is tracked at the field level: each field may be independently public, private, or other[^1].

# Structs Overview

Structs are defined using the `struct` keyword, followed by the name of the type the struct is describing.
The name may then be followed by curly braces (for a standard struct), parentheses (for a tuple struct), or a semicolon (for a unit struct).

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

# Struct Flavors

Structs come in three flavors: structs with named fields (standard structs), tuple structs, and unit structs.

Structs are very much like **Tuples**. In fact, the only difference between structs and tuples has to do with what is anonymous. The general form goes like this:

- Structs are named and have named fields:

  ```rust
  struct Foo {
      bar: Bar,
      bat: Bat,
  }
  ```

  Fields are accessed by name:

  ```rust
  let bar = foo.bar;
  ```

- Tuple structs are named and have anonymous fields:

  ```rust
  Foo(Bar, Bat)
  ```

  Fields are accessed by unpacking or by numeric index:

  ```rust
  // unpacking
  let (bar, bat) = foo;
  ```

  ```rust
  // numeric index
  let bar = foo.0;
  let bat = foo.1;
  ```

- Tuples are anonymous and have anonymous fields:

  ```rust
  (Bar, Bat)
  ```

  As in tuple structs, fields are accessed by unpacking or by numeric index.

- Anonymous structs are, as the name suggests, anonymous, but with named fields. You may be familiar with them from other languages, such as Go or Java. They do not exist in Rust; it is considered idiomatic to use a tuple instead.

# Zero Sized Types

Structs can have as many or as few fields as is required. The implication is that it is legal for a struct to have zero fields. A struct with zero fields is called a **Zero-Sized Type** and has some special handling by the compiler: zero-sized types (often abbreviated as **ZSTs**) never consume stack or heap space, even when you might expect them to. For example, `Box<T>` doesn't allocate any heap memory if `T` is a ZST.

You can use standard or tuple struct syntax when writing a ZST:

```rust
// declaration
struct Foo {}
// instantiation
let foo = Foo {}
```

```rust
// declaration
struct Foo();
// instantiation
let foo = Foo();
```

There's also a shorthand for ZSTs:

```rust
// declaration
struct Foo;
// instantiation
let foo = Foo;
```

Typically, when writing a ZST, the shorthand is preferred, as you can't mix and match: whatever style was used to declare a ZST[^2] is the same style which is required to instantiate it.

After all this discussion of the syntax of ZSTs, let's talk about why would you ever want such a thing. A ZST can't store any data, but it's great for implementing traits on.

---

[^1]: [These docs](https://doc.rust-lang.org/reference/visibility-and-privacy.html) explain some less-commonly-used visibility options.
[^2]: This is a particular instance of a general rule: whatever style was used to declare any struct type is the same style required to instantiate it.
