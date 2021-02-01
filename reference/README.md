# Rust reference

## Concepts

The Rust concept exercises teach concepts from fundamentals through language mastery. They are classified and listed below.

Exercism concepts exist to serve learners in practical ways so they differ from abstract ideas. This document defines what these concepts mean in Rust. We added visual grouping below to help track maintainers create an effective reference document. Such groupings _do not_ constitute pedagogical divisions. So please do not reference them while teaching.

Checkboxes indicate whether concepts are yet taught in the exercise mapping list.

### Fundamentals of Rust

These concepts are irreducible.

- [x] Functions
- [x] Test Suites
- [x] Variable Assignment / `let`
- [x] Conditionals
- [x] Slices
- [x] Loops
- [x] Iterator usage

### Primitives

Basic primitive types:

- [x] Integers
  - [x] Signedness
- [x] Floats
- [x] Casting / `as`
- [x] Booleans
  - [x] Logical Operators (`&&`, `||`, and `!`)
- [x] `char`
- [x] `&str`

### Variables

Slightly more advanced things to do with variables.

- [ ] [Variable shadowing](https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html?#shadowing)
- [ ] `const` and `static`
- [x] Immutability / Explicit Mutability
- [x] `if let` / `while let`

### Advanced Datatypes

These concepts help organize data in more complicated ways than primitives allow.

- [x] Structs
- [ ] implementing generic structs
- [x] Tuples (anonymous structs)
- [x] Tuple Structs
- [ ] Zero-Size Structs
- [x] Enums
- [x] `Vec<T>`
- [x] `Hashmap<K, V>`
  - [x] Entry API
- [x] `HashSet<K>`
- [x] `BTreeMap`
- [ ] `FromIterator`, `IntoIterator`
- [ ] `Rc<T>`
- [ ] `Cell<T>` / `RefCell<T>`

### Intermediate Concepts

These concepts are more advanced than the fundamentals, but may be familiar from other languages.

- [x] Scopes and Expressions (implicit return)
- [x] Explicit `return`
- [x] Methods / `impl` blocks
- [x] Basics of [Generics](https://doc.rust-lang.org/book/ch10-00-generics.html)
- [ ] Using External Crates / Libraries
  - [ ] `use` statement for importing
- [x] `match` basics: ints and enums
- [ ] Destructuring (in `match`, in `let`)
- [ ] [`match` with Pattern matching](https://doc.rust-lang.org/book/ch06-02-match.html)
- [ ] Anonymous functions
- [ ] Ranges
- [ ] Closures
- [ ] Visibility / `pub`
- [ ] Typedefs / `type` keyword
- [ ] Newtype pattern

### Iterator Features

- [x] `collect`
- [ ] `count`, `sum`, turbofish
- [ ] `map`, `filter`
- [ ] `fold`
- [ ] lazy evaluation

### Unusual Language Features

These concepts are distinctive or unique to Rust.

- [x] `String`
  - [x] `format!` macro
- [x] `String` vs `&str`
- [x] `Option<T>`
- [x] `Result<T, E>`
- [x] Implementing an external (stdlib) trait, i.e. `Iterator`

### Ownership

- [ ] basics / move semantics
- [ ] references
- [ ] many immutable or one mutable reference
- [ ] `AsRef` trait / deref coercion
- [ ] `Borrow` trait
- [ ] `'static` lifetime
- [ ] generic lifetimes (`'a`)
- [ ] anonymous lifetime

### Traits

- [x] Using external traits as bounds
- [x] Deriving (`PartialEq`, `PartialOrd`, `Debug`, `Display`, `Default`)
  - [ ] Total Ordering: `Eq` and `Ord`
- [x] Manually implementing a trait
- [ ] Numeric Traits (`Add`, `Sub`, `Mul`, `Div`)
  - [ ] overflow/underflow functions / traits
- [ ] Designing a Custom Trait

### Advanced Concepts

- [ ] Writing declarative macros
- [ ] Writing procedural macros
- [ ] `Box<T>`
- [ ] `unsafe`
- [ ] Higher-order functions / `Fn`, `FnMut`, and `FnOnce`
- [ ] async

### Parallelism

- [ ] `std::thread`
- [ ] Channels (crossbeam)
- [ ] `Arc<T>`
- [ ] `Mutex<T>`
- [ ] `RwLock<T>`
- [ ] Futures (async-std)

## Concept exercise mapping

While the ideal is to have a single concept exercise for each of the concepts listed above, which
teaches exactly one concept and requires only its prerequisites,
doing so is not entirely practical. For example, it would be difficult to design an exercise
which taught what functions are without requiring any knowledge of primitives; likewise,
it would be difficult to teach primitives, with a test suite, without any knowledge of functions.

The exercises in this table should form a DAG which will eventually cover all concepts listed above.

| exercise                 | prerequisites                                | teaches                                                  | narrative                                                                        |
| ------------------------ | -------------------------------------------- | -------------------------------------------------------- | -------------------------------------------------------------------------------- |
| `hello-world`            | -                                            | functions, test suites                                   | return the `&str` literal `"hello world"`                                        |
| `compiled-calculator`    | `hello-world`                                | integers, variable assignment                            | perform a set of math operations and implicitly return the result                |
| `conditionals`           | `hello-world`                                | conditionals                                             | should structure as 3-branch `if` / `else if` / `else`; implicit return          |
| `slices`                 | `hello-world`, `compiled-calculator`         | slices                                                   | extract a slice from a parameter which is a slice of ints                        |
| `loops`                  | `slices`                                     | basic looping over a slice                               | ideally with early return so it's cumbersome to express as an iterator           |
| `iterator-use`           | `slices`                                     | iterator usage, `collect`                                |
| `integer-sizes`          | `compiled-calculator`                        | non-`i32` integers, casting                              | still all signed                                                                 |
| `unsigned-ints`          | `integer-sizes`                              | unsigned ints                                            |
| `indexing`               | `integer-sizes`, `slices`                    | indexing, `usize`                                        |
| `floats`                 | `compiled-calculator`                        | floats                                                   | probably the same compiled calculator exercise, with non-int results             |
| `bools`                  | `compiled-calculator`                        | bools, boolean operators                                 |
| `char`                   | `compiled-calculator`                        | `char` datatype                                          | should include some wide char examples                                           |
| `str-use`                | `char`, `slices`                             | `&str`                                                   | note: immutable!                                                                 |
| `string-use`             | `str-use`                                    | `String`                                                 | note: owned, mutable                                                             |
| `format-macro`           | `string-use`                                 | `format!` macro                                          |
| `str-vs-string`          | `str-use`, `string-use`                      | combining, converting, costs, etc                        |
| `generics-basics`        | `integer-sizes`, `floats`                    | using external traits as bounds, basic generics          | probably still compiled-calculator, just generic                                 |
| `structs`                | `string-use`, `unsigned-ints`, `floats`      | creating and using a struct to manage data               | `Person` has name, height, and qty shoes owned                                   |
| `implicit-return`        | `compiled-calculator`                        | scopes, expressions, implicit return                     |
| `explicit-return`        | `implicit-return`, `conditionals`            | explicit `return` statement                              |
| `methods`                | `structs`, `explicit-return`                 | `impl` blocks, methods                                   |
| `tuples`                 | `structs`                                    | tuples                                                   | teach as anonymous structs; teach `.0` syntax                                    |
| `tuple-structs`          | `tuples`, `structs`                          | tuple structs (named tuples)                             |
| `basic-enums`            | `unsigned-ints`                              | basic enums (no fields), `match`                         |
| `enums`                  | `basic-enums`, `tuple-structs`               | tuple syntax for enum variants                           |
| `option`                 | `enums`                                      | `Option<T>`                                              |
| `result`                 | `enums`                                      | `Result<T, E>`                                           |
| `mutability`             | `compiled-calculator`                        | immutability by default; explicit mutability             |
| `let-expressions`        | `loops`, `option`, `result`                  | `if let`, `while let`                                    | probably will only depend on one of `option` or `result` based on implementation |
| `vectors`                | `slices`                                     | `Vec<T>`                                                 |
| `hashmaps`               | `vectors`                                    | `Hashmap<K, V>`                                          |
| `entry-api`              | `hashmaps`                                   | Entry API                                                |
| `hashsets`               | `hashmaps`                                   | `HashSet<K>`                                             |
| `btreemaps`              | `hashmaps`                                   | `BTreeMap<K, V>`                                         |
| `deriving-traits`        | `structs`                                    | `PartialEq`, `PartialOrd`, `Debug`, `Display`, `Default` |
| `implementing-traits`    | `deriving-traits`                            | manually implementing a trait                            | probably implement `Iterator`                                                    |
| `collection-conversions` | `vectors`, `hashsets`, `implementing-traits` | `FromIterator`, `IntoIterator`                           |
