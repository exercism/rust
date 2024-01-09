## General

- Instead of implementing arbitrary-precision arithmetic from scratch, consider building your type on top of the [num_bigint](https://crates.io/crates/num-bigint) crate.
- You might be able to [derive](https://doc.rust-lang.org/book/2018-edition/appendix-03-derivable-traits.html) some of the required traits.
- `Decimal` is assumed to be a signed type. You do not have to create a separate unsigned type, though you may do so as an implementation detail if you so choose.
