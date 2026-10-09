## General

- Ranking a list of poker hands can be considered a sorting problem.
- Rust provides the [sort](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.sort) method for `Vec<T> where T: Ord`.
- You might consider implementing a type representing a poker hand which implements [`Ord`](https://doc.rust-lang.org/std/cmp/trait.Ord.html).
