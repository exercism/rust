# Concepts Required for the Poker exercise

There are many ways to potentially solve an exercise, but I'll be looking at the [canonical example](https://github.com/exercism/rust/blob/c0afe3855e0e71dc5429aa97657e074aa90a8e11/exercises/poker/example.rs) as my primary reference for what concepts are used.

When referring to the list of concepts, we mean [this list](https://github.com/exercism/v3/blob/master/languages/rust/reference/README.md)

Note that for both concept and practice exercises, the list of prerequisite concepts should be as small as possible. The goal here is not to enumerate all concepts which may be used in all possible approaches to the problem; it is to enumerate the minimal set of knowledge with which a determined student could conceivably solve the problem.

## Required concepts

### Existing

- fundamentals
- `String` vs `&str`
- Ownership / Borrowing
- `Option<T>`
- `Vec<T>`
- `Traits`
- `Enums`

### Should be added

- Lifetimes

## Optional Concepts

### Existing

- `Eq` and `Ord`: the [`README`](https://github.com/exercism/rust/tree/c0afe3855e0e71dc5429aa97657e074aa90a8e11/exercises/poker#hints) strongly suggests that we implement this in terms of `PartialOrd`, which is covered by this concept. However, nothing in the exercise design makes it necessary to actually implement `Eq` or `Ord`.
- Using External Crates / Libraries: the existing implementation makes use of the `counter` library
- `Hashmap<K, V>`: the `counter` library fundamentally just provides a newtype adding some utility methods to a hashmap

### Should be added

- Parsing and `FromStr`

## Open Questions

- Which, if any, of the optional concepts should be promoted into the actual list of prerequisites?
