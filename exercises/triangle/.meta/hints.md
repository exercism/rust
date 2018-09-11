# Triangle in Rust

- [Result](https://doc.rust-lang.org/std/result/index.html)

Implementation of this can take many forms. Here are some topics that may help you, depending on the approach you take.

- [Enums](https://doc.rust-lang.org/book/2018-edition/ch06-00-enums.html)
- [Traits](https://doc.rust-lang.org/book/2018-edition/ch10-02-traits.html)
- [BTreeSet](https://doc.rust-lang.org/std/collections/btree_set/struct.BTreeSet.html)

Or maybe you will come up with an approach that uses none of those!

## Non-integer lengths

The base exercise tests identification of triangles whose sides are all
integers. However, some triangles cannot be represented by pure integers. A simple example is a right triangle (an isosceles triangle whose equal sides are separated by 90 degrees) whose equal sides both have length of 1. Its hypotenuse is the square root of 2, which is an irrational number: no simple multiplication can represent this number as an integer.

It would be tedious to rewrite the analysis functions to handle both integer and floating-point cases, and particularly tedious to do so for all potential integer and floating point types: given signed and unsigned variants of bitwidths 8, 16, 32, 64, and 128, that would be 10 reimplementations of fundamentally the same code even before considering floats!

There's a better way: [generics](https://doc.rust-lang.org/stable/book/2018-edition/ch10-00-generics.html). By rewriting your Triangle as a `Triangle<T>`, you can write your code once, and hand off the work of generating all those specializations to the compiler. Note that in order to use mathematical operations, you'll need to constrain your generic type to types which support those operations using traits.

There are some bonus tests you can run which test your implementation on floating-point numbers. To enable them, run your tests with the `generic` feature flag, like this:

```bash
cargo test --features generic
```
