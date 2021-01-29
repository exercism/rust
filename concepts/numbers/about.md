# Numbers

The very word "computer" derives from "computation", meaning an operation performed on numbers. Computers, and Rust, are highly optimized for performing calculations on numbers.

Rust has several different primitive numeric types, which are appropriate for different use cases. The most basic choice is between [integers](../integers/about.md) and [floating-point numbers](../floating-point-numbers/about.md). There are also a variety of external crates providing non-primitive numeric types for specialized uses.

Rust defines several comparison and mathematical operators which are applied to numbers. If desired, you can apply these operators to your own type by implementing the appropriate trait. The appropriate traits are defined in [`std::ops`](https://doc.rust-lang.org/std/ops/index.html#traits) and [`std::cmp`](https://doc.rust-lang.org/std/cmp/index.html#traits).

For example, if you define a custom type representing a filesystem path[^1], you might implement path joining by means of the `/` operator by implementing [`std::ops::Div`](https://doc.rust-lang.org/std/ops/trait.Div.html).

---

[^1]: You should probably use `std::path::Path` instead of defining your own filesystem path type.
