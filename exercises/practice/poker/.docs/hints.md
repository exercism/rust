## General

- Ranking a list of poker hands can be considered a sorting problem.
- Rust provides the [sort](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.sort) method for `Vec<T> where T: Ord`.
- [`Ord` types](https://doc.rust-lang.org/std/cmp/trait.Ord.html) form a [total order](https://en.wikipedia.org/wiki/Total_order): exactly one of `a < b`, `a == b`, or `a > b` must be true.
- Poker hands do not conform to a total order: it is possible for two hands to be non-equal but have equal sort order. Example: `"3S 4S 5D 6H JH"`, `"3H 4H 5C 6C JD"`.
- Rust provides the [`PartialOrd` trait](https://doc.rust-lang.org/std/cmp/trait.PartialOrd.html) to handle the case of sortable things which do not have a total order. However, it doesn't provide a standard `sort` method for `Vec<T> where T: PartialOrd`. The standard idiom to sort a vector in this case is `your_vec.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::{Less|Equal|Greater}));`, depending on your needs.
- You might consider implementing a type representing a poker hand which implements `PartialOrd`.
